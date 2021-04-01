use std::boxed::Box;
use std::fmt;
use std::vec::Vec;

use rocket::State;

use crate::model::*;

mod gcp_datastore;
mod mem;
mod sqlite;

pub use gcp_datastore::GcpDsInterface as GcpDs;
pub use mem::MemInterface as Mem;
pub use sqlite::SqliteInterface as Sqlite;

#[derive(Debug)]
pub enum Error {
    CircuitIdNotFound(CircuitId),
    BadResponse(&'static str),
    Other(Box<dyn std::error::Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CircuitIdNotFound(id) => write!(f, "Invalid circuit id: {}", id),
            Error::BadResponse(msg) => write!(f, "TODO: Make an error for GCP: {}", msg),
            Error::Other(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

pub trait Interface: Send + Sync {
    fn load_circuit(&self, id: CircuitId) -> Result<Circuit>;
    fn enumerate_circuits(&self, id: UserId) -> Result<Vec<CircuitMetadata>>;
    fn update_circuit(&self, c: &Circuit) -> Result<()>;
    fn new_circuit(&self, c: Circuit) -> Result<Circuit>;
    fn delete_circuit(&self, id: CircuitId) -> Result<()>;
}

pub type Storage<'r> = State<'r, Box<dyn Interface>>;
