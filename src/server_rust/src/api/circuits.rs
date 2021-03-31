use rocket_contrib::json::Json;

use rocket::response::status::NotFound;

use crate::auth::UserToken;
use crate::model::*;
use crate::storage::Storage;

// TODO: loading the circuit metadata should happen in a guard,
//  and access control should happen in a guard as well
fn load_circuit(s: &Storage, user: &UserToken, id: CircuitId) -> Result<Circuit, NotFound<()>> {
    let circuit = match s.load_circuit(id) {
        Ok(c) => c,
        Err(_) => return Err(NotFound(())),
    };

    if circuit.metadata.owner != user.id {
        return Err(NotFound(()));
    }
    Ok(circuit)
}

#[get("/circuits")]
pub fn enumerate(s: Storage, key: UserToken) -> Json<Vec<CircuitMetadata>> {
    Json(s.enumerate_circuits(key.id).unwrap())
}

#[get("/circuits/<id>")]
pub fn get(s: Storage, user: UserToken, id: CircuitId) -> Result<Json<Circuit>, NotFound<()>> {
    Ok(Json(load_circuit(&s, &user, id)?))
}

#[post("/circuits", data = "<circuit>")]
pub fn create(s: Storage, user: UserToken, circuit: Json<Circuit>) -> Json<CircuitMetadata> {
    let mut c: Circuit = circuit.clone();
    c.metadata.owner = user.id;

    let c = s.new_circuit(c).unwrap();
    Json(c.metadata)
}

#[put("/circuits/<id>", data = "<new>")]
pub fn update(
    s: Storage,
    user: UserToken,
    id: CircuitId,
    new: Json<Circuit>,
) -> Result<Json<CircuitMetadata>, NotFound<()>> {
    let c = load_circuit(&s, &user, id)?.update(new.clone());
    match s.update_circuit(&c) {
        Ok(_) => Ok(Json(c.metadata)),
        Err(_) => Err(NotFound(())),
    }
}

#[post("/circuits/<id>/delete")]
pub fn delete(s: Storage, user: UserToken, id: CircuitId) -> Result<Json<()>, NotFound<()>> {
    let c = load_circuit(&s, &user, id)?;
    match s.delete_circuit(c.metadata.id) {
        Ok(_) => Ok(Json(())),
        Err(_) => Err(NotFound(())),
    }
}
