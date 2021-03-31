use rocket::Route;
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
