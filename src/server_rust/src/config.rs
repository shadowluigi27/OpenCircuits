use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::auth::{GoogleOAuth2, IdentityDecoder, Method, NoAuth};
use crate::storage::{GcpDs, Interface, Mem, Sqlite};

#[derive(Serialize, Deserialize)]
pub enum StorageType {
    Mem,
    Sqlite { path: String },
    GcpEmu { host: String, proj: String },
    Gcp,
}
#[derive(Serialize, Deserialize)]
pub enum AuthType {
    NoAuth,
    GoogleOAuth(String),
}
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub storage_type: StorageType,
    pub auth_types: Vec<AuthType>,
    pub address: String,
    pub port: u16,
}

pub fn load_config<T: Default + Serialize + for<'de> Deserialize<'de>, P: AsRef<Path>>(
    path: P,
) -> Result<T, Box<dyn Error>> {
    match File::open(&path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            Ok(serde_json::from_reader(reader)?)
        }
        Err(_) => {
            let cfg = T::default();
            let file = File::create(path)?;
            serde_json::to_writer_pretty(BufWriter::new(file), &cfg)?;
            Ok(cfg)
        }
    }
}

impl StorageType {
    fn make(self) -> Box<dyn Interface> {
        match self {
            Self::Mem => Box::new(Mem::new()),
            Self::Sqlite { path } => Box::new(Sqlite::new(&path).unwrap()),
            Self::GcpEmu { host, proj } => Box::new(GcpDs::new_emu(proj, host)),
            Self::Gcp => Box::new(GcpDs::new(std::env::var("DATASTORE_PROJECT_ID").unwrap())),
        }
    }
}
impl AuthType {
    fn make(self) -> Box<dyn Method> {
        match self {
            Self::NoAuth => Box::new(NoAuth::new()),
            Self::GoogleOAuth(cfg) => Box::new(GoogleOAuth2::new(cfg).unwrap()),
        }
    }
}
impl Config {
    pub fn make(self) -> (Box<dyn Interface>, IdentityDecoder) {
        let storage = self.storage_type.make();
        let mut v = Vec::new();
        for a in self.auth_types {
            v.push(a.make());
        }
        (storage, IdentityDecoder::new(v))
    }
}
impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            storage_type: StorageType::Mem,
            auth_types: vec![AuthType::NoAuth],
            address: String::from("0.0.0.0"),
            port: 8080,
        }
    }
}
