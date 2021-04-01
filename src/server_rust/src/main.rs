#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;

use std::boxed::Box;
use std::error::Error;

use config::{load_config, Config};

mod api;
mod auth;
mod config;
mod model;
mod storage;
mod web;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cfg: Config = load_config("open_circuits.json")?;

    // When in production, it may be appropriate to force some config values
    let is_gcp = std::env::var("DATASTORE_PROJECT_ID").is_ok();
    if is_gcp {
        cfg.storage_type = config::StorageType::Gcp;
        cfg.auth_types = vec![config::AuthType::GoogleOAuth(String::from(
            "credentials.json",
        ))];
    }

    let (storage, identifier) = cfg.make();

    rocket::ignite()
        .mount("/api", api::routes())
        .mount("/", web::routes())
        .manage(identifier)
        .manage(storage)
        .launch();
    Ok(())
}
