use std::vec::Vec;

use rocket::{Catcher, Route};
use rocket_contrib::json::Json;

use crate::auth::UserToken;

mod circuits;

#[get("/ping")]
fn auth_ping(key: UserToken) -> Json<String> {
    Json(format!("Ping successful for user id {:?}", key.0))
}

pub fn routes() -> Vec<Route> {
    routes![
        auth_ping,
        circuits::enumerate,
        circuits::get,
        circuits::create,
        circuits::update,
        circuits::delete
    ]
}

#[catch(400)]
fn c400() -> () {}
#[catch(401)]
fn c401() -> () {}
#[catch(403)]
fn c403() -> () {}
#[catch(404)]
fn c404() -> () {}
#[catch(405)]
fn c405() -> () {}

#[catch(500)]
fn c500() -> () {}

pub fn catchers() -> Vec<Catcher> {
    // Override default catchers to NOT return HTML
    catchers!(c400, c401, c403, c404, c405, c500)
}
