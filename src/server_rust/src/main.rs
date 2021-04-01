#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;

use std::boxed::Box;
use std::error::Error;

use rocket::config::{ConfigBuilder, Environment};

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

    let env = if is_gcp {
        Environment::Production
    } else {
        Environment::Development
    };
    let rocket_config = ConfigBuilder::new(env)
        .address(cfg.address.clone())
        .port(cfg.port)
        .finalize()?;

    let (storage, identifier) = cfg.make();

    rocket::custom(rocket_config)
        .mount("/api", api::routes())
        .register(api::catchers())
        .mount("/", web::routes())
        .manage(identifier)
        .manage(storage)
        .launch();
    Ok(())
}
