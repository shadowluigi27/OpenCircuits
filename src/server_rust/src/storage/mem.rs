use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::model::*;
use crate::storage::{Error, Interface, Result};

struct MemDataInner {
    map: HashMap<CircuitId, Circuit>,
    idx: u32,
}

pub struct MemInterface {
    data: Arc<RwLock<MemDataInner>>,
}

impl MemInterface {
    #[allow(dead_code)]
    pub fn new() -> MemInterface {
        MemInterface {
            data: Arc::new(RwLock::new(MemDataInner {
                map: HashMap::default(),
                idx: 0,
            })),
        }
    }
}

impl Interface for MemInterface {
    fn load_circuit(&self, id: CircuitId) -> Result<Circuit> {
        let guard = self.data.read().expect("Poisoned");
        match guard.map.get(&id) {
            Some(c) => Ok(c.clone()),
            None => Err(Error::CircuitIdNotFound(id)),
        }
    }
    fn enumerate_circuits(&self, id: UserId) -> Result<Vec<CircuitMetadata>> {
        let guard = self.data.read().expect("Poisoned");
        let m: &MemDataInner = &guard;

        let mut circs = Vec::new();
        for (_, c) in m.map.iter() {
            if c.metadata.owner == id {
                circs.push(c.metadata.clone());
            }
        }
        Ok(circs)
    }
    fn update_circuit(&self, c: &Circuit) -> Result<()> {
        let mut guard = self.data.write().expect("Poisoned");
        let m: &mut MemDataInner = &mut *guard;
        m.map.insert(c.metadata.id.clone(), c.clone());
        Ok(())
    }
    fn new_circuit(&self, mut c: Circuit) -> Result<Circuit> {
        let mut guard = self.data.write().expect("Poisoned");
        let m: &mut MemDataInner = &mut *guard;
        m.idx += 1;

        // Create a new circuit, return the original with an id added
        let new_id = format!("{}", m.idx);
        c.metadata.id = new_id.clone();
        m.map.insert(new_id, c.clone());
        Ok(c)
    }
    fn delete_circuit(&self, id: CircuitId) -> Result<()> {
        let mut guard = self.data.write().expect("Poisoned");
        let m: &mut MemDataInner = &mut *guard;

        match m.map.remove(&id) {
            Some(_) => Ok(()),
            None => Err(Error::CircuitIdNotFound(id)),
        }
    }
}
