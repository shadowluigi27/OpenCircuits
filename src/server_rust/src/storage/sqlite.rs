use std::sync::{Arc, Mutex};

use rusqlite::{Connection, NO_PARAMS};

use crate::model::*;
use crate::storage::{Error, Interface, Result};

// NOTE: Sqlite is a developer-only feature and is volatile.
//  No migrations will be supported

#[derive(Clone)]
pub struct SqliteInterface {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteInterface {
    pub fn new(path: &str) -> rusqlite::Result<SqliteInterface> {
        let conn = Connection::open(path)?;
        // Check the table exists
        match conn.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='circuits'",
            NO_PARAMS,
            |_| Ok(()),
        ) {
            Ok(o) => Ok(o),

            // Create the table if it doesn't exist
            Err(rusqlite::Error::QueryReturnedNoRows) => conn
                .execute(
                    "CREATE TABLE circuits (
                        id TEXT PRIMARY KEY UNIQUE,
                        name TEXT NOT NULL,
                        owner TEXT NOT NULL,
                        desc TEXT NOT NULL,
                        version TEXT NOT NULL,
                        thumbnail TEXT NOT NULL,
                        contents TEXT NOT NULL
                    )",
                    NO_PARAMS,
                )
                .map(|_| ()),
            Err(e) => Err(e),
        }?;

        Ok(SqliteInterface {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

impl Interface for SqliteInterface {
    fn load_circuit(&self, id: CircuitId) -> Result<Circuit> {
        let conn = self.conn.lock().unwrap();
        let mut load = conn.prepare(
            "SELECT id, name, owner, desc, version, thumbnail, contents FROM circuits WHERE id=?",
        ).map_err(|e| Error::Internal(e.into()))?;
        match load.query_row(&[id.clone()], |row| {
            Ok(Circuit {
                metadata: CircuitMetadata {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    owner: row.get(2)?,
                    desc: row.get(3)?,
                    version: row.get(4)?,
                    thumbnail: row.get(5)?,
                },
                contents: row.get(6)?,
            })
        }) {
            Err(rusqlite::Error::QueryReturnedNoRows) => Err(Error::CircuitIdNotFound(id)),
            Err(o) => Err(Error::Internal(o.into())),
            Ok(c) => Ok(c),
        }
    }
    fn enumerate_circuits(&self, id: UserId) -> Result<Vec<CircuitMetadata>> {
        let conn = self.conn.lock().unwrap();
        let mut query = conn
            .prepare("SELECT id, name, owner, desc, version, thumbnail FROM circuits WHERE owner=?")
            .map_err(|e| Error::Internal(e.into()))?;
        let res = query
            .query_map(&[id], |row| {
                Ok(CircuitMetadata {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    owner: row.get(2)?,
                    desc: row.get(3)?,
                    version: row.get(4)?,
                    thumbnail: row.get(5)?,
                })
            })
            .map_err(|e| Error::Internal(e.into()))?;

        let mut entries = Vec::new();
        for r in res {
            let v: CircuitMetadata = r.map_err(|e| Error::Internal(e.into()))?;
            entries.push(v);
        }
        Ok(entries)
    }
    fn update_circuit(
        &self,
        Circuit {
            metadata:
                CircuitMetadata {
                    id,
                    name,
                    owner,
                    desc,
                    version,
                    thumbnail,
                },
            contents,
        }: &Circuit,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let mut store = conn.prepare(
            "UPDATE circuits SET name=?, owner=?, desc=?, version=?, thumbnail=?, contents=? WHERE id=?",
        ).map_err(|e| Error::Internal(e.into()))?;
        match store.execute(&[name, owner, desc, version, thumbnail, contents, id]) {
            Ok(1) => Ok(()),
            Ok(_) => Err(Error::CircuitIdNotFound(id.clone())),
            Err(e) => Err(Error::Internal(e.into())),
        }
    }
    fn new_circuit(&self, mut c: Circuit) -> Result<Circuit> {
        let conn = self.conn.lock().unwrap();
        let mut create = conn.prepare(
            "INSERT INTO circuits(id, name, owner, desc, version, thumbnail, contents) VALUES (?, ?, ?, ?, ?, ?, ?)"
        ).map_err(|e| Error::Internal(e.into()))?;

        // This is NOT OK for production, because SystemTime is not monotone,
        //  and the circuit ID should not have meaningful information in it
        let ns = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let id = format!("{}", ns);
        c.metadata.id = id;

        let Circuit {
            metadata:
                CircuitMetadata {
                    id,
                    name,
                    owner,
                    desc,
                    version,
                    thumbnail,
                },
            contents,
        } = &c;
        match create.insert(&[id, name, owner, desc, version, thumbnail, contents]) {
            Ok(_) => Ok(c),
            Err(e) => Err(Error::Internal(e.into())),
        }
    }
    fn delete_circuit(&self, id: CircuitId) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let mut delete = conn
            .prepare("DELETE FROM circuits WHERE id=?")
            .map_err(|e| Error::Internal(e.into()))?;
        match delete.execute(&[&id]) {
            Ok(1) => Ok(()),
            Ok(0) => Err(Error::CircuitIdNotFound(id)),
            Ok(_) => panic!("Expected single deleted row"),
            Err(e) => Err(Error::Internal(e.into())),
        }
    }
}
