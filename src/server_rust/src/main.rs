#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;

use std::boxed::Box;
use std::vec::Vec;

mod api;
mod auth;
mod model;
mod storage;
mod web;

fn main() {
    let mut auths: Vec<Box<dyn auth::AuthenticationMethod>> = std::vec::Vec::new();

    // TODO: Use a json file or something for configuration, then check env vars for production
    let is_gcp = std::env::var("DATASTORE_PROJECT_ID").is_ok();

    if std::env::var("NO_AUTH").is_ok() {
        auths.push(Box::new(auth::no_auth::NoLoginAuthProvider::new()))
    }

    let storage: Box<dyn storage::Interface> = if is_gcp {
        panic!("GCP Not supported yet");
    } else {
        if let Ok(v) = std::env::var("STORAGE") {
            match v.as_str() {
                "sqlite" => Box::new(storage::sqlite::SqliteInterface::new("circuits.db").unwrap()),
                "gcp_emu" => panic!("GCP Emulator not supported yet"),
                _ => Box::new(storage::mem::MemInterface::new()),
            }
        } else {
            Box::new(storage::mem::MemInterface::new())
        }
    };

    let auth_manager = auth::AuthenticationManager::new(auths);

    rocket::ignite()
        .mount("/api", api::routes())
        .mount("/", web::routes())
        .manage(auth_manager)
        .manage(storage)
        .launch();
}
