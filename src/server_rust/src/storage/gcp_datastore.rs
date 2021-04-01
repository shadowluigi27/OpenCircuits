extern crate google_datastore1 as datastore1;
extern crate hyper;
extern crate yup_oauth2 as oauth2;

use std::collections::{hash_map::RandomState, HashMap};
use std::default::Default;
use std::iter::FromIterator;
use std::{error, fmt, result};

use datastore1::{BeginTransactionRequest, Datastore, EntityResult, Key, MutationResult, Value};

use crate::model::*;
use crate::storage::{Error as SError, Interface, Result as SResult};

mod requests;

#[derive(Debug)]
pub enum Error {
    CircuitIdNotFound(CircuitId),
    MissingKey,
    MalformedKey(&'static str),
    MissingEntity,
    MalformedEntity(&'static str),
    MissingTransaction,
    MissingMutations,
    UnexpectedMutations,
    MissingResults,
    UnexpectedResults,
    ReqError(datastore1::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Debug).fmt(f)
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::ReqError(ref e) => Some(e),
            _ => None,
        }
    }
}
impl Error {
    fn wrap(self) -> SError {
        match self {
            Error::CircuitIdNotFound(id) => SError::CircuitIdNotFound(id),
            a => SError::Internal(Box::new(a)),
        }
    }
}
pub type Result<T> = result::Result<T, Error>;

fn get_id_from_key(key: Option<Key>) -> Result<String> {
    key.ok_or(Error::MissingKey)?
        .path
        .ok_or(Error::MalformedKey("Missing path"))?
        .get(0)
        .ok_or(Error::MalformedKey("Missing path entry"))?
        .id
        .clone()
        .ok_or(Error::MalformedKey("Missing id"))
}

// All places that need to be explicitly updated when the circuit model changes
//  will be denoted with a CHANGE_ME tag

// CHANGE_ME when editing the model
fn format_circuit(c: Circuit) -> HashMap<String, Value> {
    HashMap::from_iter(
        vec![
            ("Name", (c.metadata.name, false)),
            ("Owner", (c.metadata.owner, false)),
            ("Desc", (c.metadata.desc, false)),
            ("Thumbnail", (c.metadata.thumbnail, true)),
            ("Version", (c.metadata.version, false)),
            ("CircuitDesigner", (c.contents, true)),
        ]
        .iter()
        .map(|(k, (v, e))| {
            (
                String::from(*k),
                Value {
                    string_value: Some(v.clone()),
                    exclude_from_indexes: Some(*e),
                    ..Value::default()
                },
            )
        }),
    )
}

// CHANGE_ME when editing the model
fn parse_circuit(result: EntityResult) -> Result<(CircuitMetadata, Option<String>)> {
    let entity = result.entity.ok_or(Error::MissingEntity)?;

    let id = get_id_from_key(entity.key)?;

    let props: HashMap<String, Value> = entity
        .properties
        .ok_or(Error::MalformedEntity("Empty props"))?;

    let str_props: HashMap<&str, String, RandomState> = HashMap::from_iter(
        props
            .iter()
            .map(|(k, v)| (k, v.string_value.clone()))
            .filter(|(_, v)| v.is_some())
            .map(|(k, v)| (k.as_str(), v.unwrap())),
    );

    let parse_prop = |k: &str| -> Result<String> {
        Ok(str_props
            .get(k)
            .ok_or(Error::MalformedEntity("Incomplete entry"))?
            .clone())
    };

    let metadata = CircuitMetadata {
        id: id,
        name: parse_prop("Name")?,
        owner: parse_prop("Owner").unwrap_or_default(),
        desc: parse_prop("Desc")?,
        thumbnail: parse_prop("Thumbnail")?,
        version: parse_prop("Version")?,
    };
    let contents = str_props.get("CircuitDesigner").map(|w| w.clone());
    Ok((metadata, contents))
}

fn unwrap_resp<S, T>(result: result::Result<(S, T), datastore1::Error>) -> Result<T> {
    match result {
        Ok((_, res)) => Ok(res),
        Err(e) => Err(Error::ReqError(e)),
    }
}

// For now, just create a new connection for every request.
//  If that is too slow, a pooled resource approach could be added
pub struct GcpDsInterface {
    project_id: String,
    datastore_url: Option<String>,
    kind: String,
}

impl Interface for GcpDsInterface {
    fn load_circuit(&self, id: CircuitId) -> SResult<Circuit> {
        self.load_circuit_1(id).map_err(|e| e.wrap())
    }
    fn enumerate_circuits(&self, id: UserId) -> SResult<Vec<CircuitMetadata>> {
        self.enumerate_circuits_1(id).map_err(|e| e.wrap())
    }
    fn update_circuit(&self, c: &Circuit) -> SResult<()> {
        self.update_circuit_1(c).map_err(|e| e.wrap())
    }
    fn new_circuit(&self, c: Circuit) -> SResult<Circuit> {
        self.new_circuit_1(c).map_err(|e| e.wrap())
    }
    fn delete_circuit(&self, id: CircuitId) -> SResult<()> {
        self.delete_circuit_1(id).map_err(|e| e.wrap())
    }
}

// The GCP datastore crate requires an auth provider, but that is not required
//  to run using the emulator or on GCP compute engine
struct DummyAuth {}
impl oauth2::GetToken for DummyAuth {
    fn token<'b, I, T>(
        &mut self,
        _: I,
    ) -> std::result::Result<oauth2::Token, Box<dyn std::error::Error>>
    where
        T: AsRef<str> + Ord + 'b,
        I: IntoIterator<Item = &'b T>,
    {
        Ok(oauth2::Token {
            access_token: String::default(),
            refresh_token: String::default(),
            token_type: String::default(),
            expires_in: None,
            expires_in_timestamp: None,
        })
    }

    fn api_key(&mut self) -> Option<String> {
        None
    }
}

impl GcpDsInterface {
    pub fn new(project_id: String) -> GcpDsInterface {
        GcpDsInterface {
            project_id: project_id,
            datastore_url: None,
            kind: String::from("Circuit"),
        }
    }

    // NOTE: the "emu_url" MUST be fully-specified (with http and trailing slash)
    pub fn new_emu(project_id: String, emu_url: String) -> GcpDsInterface {
        GcpDsInterface {
            project_id: project_id,
            datastore_url: Some(emu_url),
            kind: String::from("Circuit"),
        }
    }

    fn get_conn(&self) -> Datastore<hyper::Client, DummyAuth> {
        let client = hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_native_tls::NativeTlsClient::new().unwrap(),
        ));

        let mut hub = Datastore::new(client, DummyAuth {});

        if let Some(url) = self.datastore_url.clone() {
            // Overwrite URL's for datastore emulator
            hub.base_url(String::from(url.clone()));
            hub.root_url(String::from(url));
        }
        hub
    }

    fn init_transaction(&self) -> Result<(Datastore<hyper::Client, DummyAuth>, String)> {
        let hub = self.get_conn();
        let result = hub
            .projects()
            .begin_transaction(BeginTransactionRequest::default(), &self.project_id)
            .doit();

        let trans = unwrap_resp(result)?
            .transaction
            .ok_or(Error::MissingTransaction)?;
        Ok((hub, trans))
    }

    fn put_circuit(&self, c: &Circuit) -> Result<MutationResult> {
        let (hub, trans) = self.init_transaction()?;
        let id = if c.metadata.id.len() > 0 {
            Some(c.metadata.id.clone())
        } else {
            None
        };
        let result = hub
            .projects()
            .commit(
                requests::put_req(&self.kind, id, trans, format_circuit(c.clone())),
                &self.project_id,
            )
            .doit();

        match unwrap_resp(result)?.mutation_results {
            Some(v) if v.len() == 1 => Ok(v[0].clone()),
            _ => Err(Error::MissingMutations),
        }
    }

    fn load_circuit_1(&self, id: CircuitId) -> Result<Circuit> {
        let hub = self.get_conn();
        let result = hub
            .projects()
            .lookup(
                requests::lookup_req(&self.kind, Some(id.clone()).into_iter()),
                self.project_id.as_str(),
            )
            .doit();

        let result = match unwrap_resp(result)?.found {
            Some(v) if v.len() == 1 => Ok(v[0].clone()),
            Some(_) => Err(Error::UnexpectedResults),
            None => Err(Error::CircuitIdNotFound(id)),
        }?;

        let (md, contents) = parse_circuit(result)?;
        Ok(Circuit {
            metadata: md,
            contents: contents.ok_or(Error::MissingResults)?,
        })
    }

    fn enumerate_circuits_1(&self, id: UserId) -> Result<Vec<CircuitMetadata>> {
        let hub = self.get_conn();

        // TODO: This should use projection once we can remove "Thumbnail" from
        //  the metadata
        let result = hub
            .projects()
            .run_query(
                requests::query_req(&self.kind, "Owner", &id, None),
                self.project_id.as_str(),
            )
            .doit();

        let entity_results = unwrap_resp(result)?
            .batch
            .ok_or(Error::MissingResults)?
            .entity_results;
        let entity_results = match entity_results {
            Some(res) => res,
            None => return Ok(Vec::new()),
        };

        let mut mds = Vec::new();
        for result in entity_results {
            let (mut md, _) = parse_circuit(result)?;
            md.owner = id.clone();
            mds.push(md);
        }
        Ok(mds)
    }

    fn update_circuit_1(&self, c: &Circuit) -> Result<()> {
        self.put_circuit(c).map(|_| ())
    }

    fn new_circuit_1(&self, c: Circuit) -> Result<Circuit> {
        let mut_res = self.put_circuit(&c)?;

        Ok(Circuit {
            metadata: CircuitMetadata {
                id: get_id_from_key(mut_res.key)?,
                ..c.metadata
            },
            ..c
        })
    }

    fn delete_circuit_1(&self, id: CircuitId) -> Result<()> {
        let (hub, trans) = self.init_transaction()?;
        let result = hub
            .projects()
            .commit(requests::del_req(&self.kind, trans, id), &self.project_id)
            .doit();

        match unwrap_resp(result)?
            .index_updates
            .ok_or(Error::MissingMutations)?
        {
            1 => Ok(()),
            0 => Ok(()),
            _ => Err(Error::UnexpectedMutations),
        }
    }
}
