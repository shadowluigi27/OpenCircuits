#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;

use std::boxed::Box;
use std::error::Error;

use config::{Config, load_config};

mod api;
mod auth;
mod model;
mod storage;
mod web;
mod config;

fn main() -> Result<(), Box<dyn Error>> {
    let is_gcp = std::env::var("DATASTORE_PROJECT_ID").is_ok();
    // When in production, it may be appropriate to force some config values
    if is_gcp {
        panic!("GCP not supported yet")
    }

    let cfg: Config = load_config("open_circuits.json")?;
    let (storage, identifier) = cfg.make();

    rocket::ignite()
        .mount("/api", api::routes())
        .mount("/", web::routes())
        .manage(identifier)
        .manage(storage)
        .launch();
    Ok(())
}