pub mod no_auth;

use rocket::http::Status;
use rocket::request::{FromRequest, Request};
use rocket::{Outcome, State};

use crate::model::UserId;

pub trait AuthenticationMethod: Send + Sync {
    fn extract_identity(&self, token: &str) -> Result<String, &'static str>;
    fn auth_header_prefix(&self) -> &'static str;
}

pub struct AuthenticationManager {
    auth_methods: Vec<Box<dyn AuthenticationMethod>>,
}

#[derive(Debug)]
pub enum ExtractAuthErr {
    UnsupportedProvider,
    Failed(&'static str),
}

impl AuthenticationManager {
    pub fn new(methods: Vec<Box<dyn AuthenticationMethod>>) -> AuthenticationManager {
        AuthenticationManager {
            auth_methods: methods,
        }
    }
    pub fn extract_identity(
        &self,
        auth_type: &str,
        auth_id: &str,
    ) -> Result<String, ExtractAuthErr> {
        for auth in self.auth_methods.iter() {
            if auth.auth_header_prefix() == auth_type {
                match auth.extract_identity(auth_id) {
                    Ok(id) => return Ok(id),
                    Err(e) => return Err(ExtractAuthErr::Failed(e)),
                }
            }
        }
        Err(ExtractAuthErr::UnsupportedProvider)
    }
}

pub struct UserToken {
    pub id: UserId,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
    type Error = ExtractAuthErr;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        // Get the user's auth provider and ID from the request headers
        let auth_type: Vec<_> = request.headers().get("authType").collect();
        let auth_id: Vec<_> = request.headers().get("authId").collect();

        if auth_type.len() != 1 || auth_id.len() != 1 {
            Outcome::Failure((
                Status::BadRequest,
                ExtractAuthErr::Failed("Single authType/authId required"),
            ))
        } else {
            let auth_manager: State<AuthenticationManager> =
                request.guard::<State<AuthenticationManager>>().unwrap();

            // Extract the identity from the auth provider / ID pair
            match auth_manager.extract_identity(auth_type[0], auth_id[0]) {
                Ok(ident) => Outcome::Success(UserToken { id: ident }),
                Err(e) => Outcome::Failure((Status::BadRequest, e)),
            }
        }
    }
}
