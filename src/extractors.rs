use crate::auth::{self, decode_jwt, PrivateClaim};
use crate::errors::ApiError;
use crate::models::user::AuthUser;
use actix_web::error::PayloadError;
use actix_web::{error, ResponseError};
// use actix_identity::RequestIdentity;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, HttpResponse};
// use futures::future::{err, ok, Ready};
use std::future::{ready, Ready};

/// Extractor for pulling the identity out of a request.
///
/// Simply add "user: AuthUser" to a handler to invoke this.
impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        println!("get UserInfo from request");
        ready({
            let auth = req.headers().get("Authorization");
            if let Some(val) = auth {
                let token = val
                    .to_str()
                    .unwrap()
                    .split("Bearer ")
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap();
                let result = auth::decode_jwt(token);
                match result {
                    Ok(data) => Ok(AuthUser {
                        id: data.user_id.to_string(),
                        email: String::from("s"),
                    }),
                    Err(e) => {
                        eprintln!("{}", e);
                        Err(error::ErrorUnauthorized("Invalid Authrization"))
                    }
                }
            } else {
                Err(error::ErrorUnauthorized("Invalid Authrization"))
            }
        })
    }
}
