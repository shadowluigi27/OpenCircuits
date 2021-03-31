use crate::auth::AuthenticationMethod;

pub struct NoLoginAuthProvider {}

impl NoLoginAuthProvider {
    pub fn new() -> NoLoginAuthProvider {
        NoLoginAuthProvider {}
    }
}

impl AuthenticationMethod for NoLoginAuthProvider {
    fn extract_identity(&self, token: &str) -> Result<String, &'static str> {
        if token.is_empty() {
            Err("User id cannot be blank in no_auth")
        } else {
            Ok(String::from("no_auth_") + token)
        }
    }
    fn auth_header_prefix(&self) -> &'static str {
        "no_auth"
    }
}
