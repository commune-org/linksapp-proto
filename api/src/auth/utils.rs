use dotenv::dotenv;

use std::env::var;

use actix_session::Session;
use actix_web::{
    http::header::{CONTENT_TYPE, LOCATION},
    HttpRequest, HttpResponse,
};
use argonautica::{Hasher, Verifier};

// use crate::auth::SessionUser;
use crate::auth::SessionUser;
use crate::errors::ServiceError;

pub fn secret_key() -> String {
    dotenv().ok();

    var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8))
}

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(secret_key().as_str())
        .hash()
        .map_err(|_| ServiceError::AuthenticationError(String::from("Could not hash password")))
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(secret_key().as_str())
        .verify()
        .map_err(|_| ServiceError::AuthenticationError(String::from("Could not verify password")))
}

pub fn is_json_request(req: &HttpRequest) -> bool {
    req.headers().get(CONTENT_TYPE).map_or(false, |header| {
        header
            .to_str()
            .map_or(false, |content_type| "application/json" == content_type)
    })
}

pub fn is_signed_in(session: &Session) -> bool {
    match get_current_user(session) {
        Ok(_) => true,
        _ => false,
    }
}

pub fn set_current_user(session: &mut Session, user: &SessionUser) -> () {
    // serializing to string is alright for this case,
    // but binary would be preferred in production use-cases.
    session
        .set("user", serde_json::to_string(user).unwrap())
        .unwrap();
}

pub fn get_current_user(session: &Session) -> Result<SessionUser, ServiceError> {
    let msg = "Could not retrieve user from session";

    session
        .get::<String>("user")
        .map_err(|_| ServiceError::AuthenticationError(String::from(msg)))
        .unwrap()
        .map_or(
            Err(ServiceError::AuthenticationError(String::from(msg))),
            |user| {
                serde_json::from_str(&user)
                    .or_else(|_| Err(ServiceError::AuthenticationError(String::from(msg))))
            },
        )
}

pub fn to_home() -> HttpResponse {
    HttpResponse::Found().header(LOCATION, "/me").finish()
}
