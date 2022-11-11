use crate::auth::user::User;
use crate::errors::ServiceError;
use actix_web::web;
use deadpool_postgres::{Client, Pool};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

impl AuthRequest {
    pub async fn authenticate(&self, pool: &Client) -> Result<Option<User>, ServiceError> {
        if let Some(user) = User::get_by_email(&self.email, pool).await? {
            if user.verify_password(&self.password).await? {
                return Ok(Some(user));
            }
        }
        Err(ServiceError::BadRequest(
            "Invalid username or password".into(),
        ))
    }
}
