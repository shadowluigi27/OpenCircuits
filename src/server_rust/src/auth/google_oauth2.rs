use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::auth::Method;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LoginProvider {
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    web: LoginProvider,
}

impl LoginProvider {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<LoginProvider> {
        let cfg: Config = serde_json::from_reader(BufReader::new(File::open(path)?))?;
        Ok(cfg.web)
    }
}

impl Method for LoginProvider {
    fn extract_identity(&self, token: &str) -> Result<String, &'static str> {
        // TODO: Switch to using something other than tokeninfo to reduce latency
        let mut client = google_signin::Client::new();
        client.audiences.push(self.client_id.clone());
        match client.verify(token) {
            Ok(id) => Ok(format!("google_{}", id.sub)),
            Err(_) => Err("Failed to validate token"),
        }
    }
    fn auth_header_prefix(&self) -> &'static str {
        "google"
    }
}
