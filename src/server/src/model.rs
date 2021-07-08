use serde::{Serialize, Deserialize};

pub type CircuitId = String;
pub type UserId = String;

// TODO: Move thumbail out of metadata, so metadata can be used like a handle
//  and the large content of the circuit doesn't get loaded until its needed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitMetadata {
    pub id: CircuitId,
    pub name: String,
    pub owner: UserId,
    pub desc: String,
    pub thumbnail: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    pub metadata: CircuitMetadata,
    pub contents: String,
}

impl CircuitMetadata {
    pub fn update(self, other: CircuitMetadata) -> CircuitMetadata {
        CircuitMetadata {
            id: self.id,
            owner: self.owner,
            ..other
        }
    }
}

impl Circuit {
    pub fn update(self, new_circuit: Circuit) -> Circuit {
        Circuit {
            contents: new_circuit.contents,
            metadata: self.metadata.update(new_circuit.metadata),
        }
    }
}
