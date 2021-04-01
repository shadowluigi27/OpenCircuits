extern crate google_datastore1 as datastore1;
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;

use std::collections::{hash_map::RandomState, HashMap};
use std::default::Default;
use std::iter::FromIterator;

use datastore1::{
    BeginTransactionRequest, CommitRequest, Datastore, Entity, EntityResult, Filter, Key,
    KindExpression, LookupRequest, Mutation, MutationResult, PathElement, PropertyFilter,
    PropertyReference, Query, RunQueryRequest, Value,
};

use crate::model::*;
use crate::storage::{Error, Interface, Result};

// All places that need to be explicitly updated when the circuit model changes
//  will be denoted with a CHANGE_ME tag

fn get_req(id: CircuitId) -> LookupRequest {
    LookupRequest {
        keys: Some(vec![Key {
            path: Some(vec![PathElement {
                kind: Some(String::from("Circuit")),
                id: Some(id),
                name: None,
            }]),
            ..Key::default()
        }]),
        ..LookupRequest::default()
    }
}

// TODO: This should exclude the circuit contents using Projection
fn enum_req(user: &UserId) -> RunQueryRequest {
    RunQueryRequest {
        query: Some(Query {
            filter: Some(Filter {
                property_filter: Some(PropertyFilter {
                    property: Some(PropertyReference {
                        name: Some(String::from("Owner")),
                    }),
                    value: Some(Value {
                        string_value: Some(user.clone()),
                        ..Value::default()
                    }),
                    op: Some(String::from("EQUAL")),
                }),
                ..Filter::default()
            }),
            kind: Some(vec![KindExpression {
                name: Some(String::from("Circuit")),
            }]),
            ..Query::default()
        }),
        ..RunQueryRequest::default()
    }
}

fn put_req(c: &Circuit, t: String) -> CommitRequest {
    let is_new = c.metadata.id.len() > 0;
    let id = if is_new {
        Some(c.metadata.id.clone())
    } else {
        None
    };
    let entity = Entity {
        properties: Some(format_circuit(c.clone())),
        key: Some(Key {
            path: Some(vec![PathElement {
                kind: Some(String::from("Circuit")),
                id: id,
                name: None,
            }]),
            ..Key::default()
        }),
    };
    let mutation = if is_new {
        Mutation {
            update: Some(entity),
            ..Mutation::default()
        }
    } else {
        Mutation {
            insert: Some(entity),
            ..Mutation::default()
        }
    };
    CommitRequest {
        transaction: Some(t),
        mutations: Some(vec![mutation]),
        ..CommitRequest::default()
    }
}

fn del_req(id: CircuitId, t: String) -> CommitRequest {
    CommitRequest {
        transaction: Some(t),
        mutations: Some(vec![Mutation {
            delete: Some(Key {
                path: Some(vec![PathElement {
                    kind: Some(String::from("Circuit")),
                    id: Some(id),
                    name: None,
                }]),
                ..Key::default()
            }),
            ..Mutation::default()
        }]),
        ..CommitRequest::default()
    }
}

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

fn get_id_from_key(key: Option<Key>) -> Result<CircuitId> {
    key.ok_or(Error::BadResponse("GCP returned empty key"))?
        .path
        .ok_or(Error::BadResponse("GCP returned empty path"))?
        .get(0)
        .ok_or(Error::BadResponse("GCP returned empty path vec"))?
        .id
        .clone()
        .ok_or(Error::BadResponse("GCP returned empty id"))
}

// CHANGE_ME when editing the model
fn parse_circuit(result: EntityResult) -> Result<(CircuitMetadata, Option<String>)> {
    let entity = result
        .entity
        .ok_or(Error::BadResponse("GCP returned empty entity"))?;

    let id = get_id_from_key(entity.key)?;

    let props: HashMap<String, Value> = entity
        .properties
        .ok_or(Error::BadResponse("GCP returned empty props"))?;

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
            .ok_or(Error::BadResponse("GCP returned incomplete entry"))?
            .clone())
    };
    let metadata = CircuitMetadata {
        id: id,
        name: parse_prop("Name")?,
        owner: parse_prop("Owner")?,
        desc: parse_prop("Desc")?,
        thumbnail: parse_prop("Thumbnail")?,
        version: parse_prop("Version")?,
    };
    let contents = str_props.get("CircuitDesigner").map(|w| w.clone());
    Ok((metadata, contents))
}

fn unwrap_resp<S, T, E: std::error::Error + 'static>(
    result: std::result::Result<(S, T), E>,
) -> Result<T> {
    match result {
        Ok((_, res)) => Ok(res),
        Err(e) => Err(Error::Other(e.into())),
    }
}

// For now, just create a new connection for every request.
//  If that is too slow, a pooled resource approach could be added
pub struct GcpDsInterface {
    project_id: String,
    datastore_url: Option<String>,
}

// The GCP datastore crate requires an auth provider, but that is not required
//  to run using the emulator or on GCP compute engine
struct DummyAuth {}
impl oauth2::GetToken for DummyAuth {
    fn token<'b, I, T>(
        &mut self,
        _: I,
    ) -> std::result::Result<yup_oauth2::Token, Box<dyn std::error::Error>>
    where
        T: AsRef<str> + Ord + 'b,
        I: IntoIterator<Item = &'b T>,
    {
        Ok(yup_oauth2::Token {
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
        }
    }

    // NOTE: the "emu_url" MUST be fully-specified (with http and trailing slash)
    pub fn new_emu(project_id: String, emu_url: String) -> GcpDsInterface {
        GcpDsInterface {
            project_id: project_id,
            datastore_url: Some(emu_url),
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
            .ok_or(Error::BadResponse("GCP failed to give transaction"))?;
        Ok((hub, trans))
    }

    fn put(&self, c: &Circuit) -> Result<MutationResult> {
        let (hub, trans) = self.init_transaction()?;
        let result = hub
            .projects()
            .commit(put_req(c, trans), &self.project_id)
            .doit();

        match unwrap_resp(result)?.mutation_results {
            Some(v) if v.len() == 1 => Ok(v[0].clone()),
            Some(_) => Err(Error::BadResponse("GCP returned non-empty mutation list")),
            None => Err(Error::BadResponse("GCP returned no mutations")),
        }
    }
}

impl Interface for GcpDsInterface {
    fn load_circuit(&self, id: CircuitId) -> Result<Circuit> {
        let hub = self.get_conn();
        let result = hub
            .projects()
            .lookup(get_req(id.clone()), self.project_id.as_str())
            .doit();

        let result = match unwrap_resp(result)?.found {
            Some(v) if v.len() == 1 => Ok(v[0].clone()),
            Some(_) => Err(Error::BadResponse(
                "GCP returned unexpected non-empty result list",
            )),
            None => Err(Error::CircuitIdNotFound(id)),
        }?;

        let (md, contents) = parse_circuit(result)?;
        Ok(Circuit {
            metadata: md,
            contents: contents.ok_or(Error::BadResponse("GCP No content"))?,
        })
    }
    fn enumerate_circuits(&self, id: UserId) -> Result<Vec<CircuitMetadata>> {
        let hub = self.get_conn();
        let result = hub
            .projects()
            .run_query(enum_req(&id), self.project_id.as_str())
            .doit();

        let entity_results = unwrap_resp(result)?
            .batch
            .ok_or(Error::BadResponse("Empty batch"))?
            .entity_results;
        let entity_results = match entity_results {
            Some(res) => res,
            None => return Ok(Vec::new()),
        };

        let mut mds = Vec::new();
        for result in entity_results {
            let (md, _) = parse_circuit(result)?;
            mds.push(md);
        }
        Ok(mds)
    }
    fn update_circuit(&self, c: &Circuit) -> Result<()> {
        self.put(c).map(|_| ())
    }
    fn new_circuit(&self, c: Circuit) -> Result<Circuit> {
        let mut_res = self.put(&c)?;

        Ok(Circuit {
            metadata: CircuitMetadata {
                id: get_id_from_key(mut_res.key)?,
                ..c.metadata
            },
            ..c
        })
    }
    fn delete_circuit(&self, id: CircuitId) -> Result<()> {
        let (hub, trans) = self.init_transaction()?;
        let result = hub
            .projects()
            .commit(del_req(id, trans), &self.project_id)
            .doit();

        match unwrap_resp(result)?
            .index_updates
            .ok_or(Error::BadResponse(""))?
        {
            1 => Ok(()),
            0 => Ok(()),
            _ => Err(Error::BadResponse("GCP Delete removed too many entries")),
        }
    }
}
