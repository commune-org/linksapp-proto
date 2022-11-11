use crate::config;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
// JWT claim
#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    // issuer
    iss: String,
    // subject
    sub: String,
    // issued at
    iat: i64,
    // expiry
    exp: i64,
    // user email
    email: String,
}

// struct to get converted to token and back
// TODO Duration::seconds, number of seconds should be in Config
impl Claim {
    pub fn with_email(email: &str) -> Self {
        let config = config::Config::from_env().unwrap();
        let domain = config.srv_cnf.host.clone();

        Self {
            iss: domain,
            sub: "auth".into(),
            email: email.into(),
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::seconds(3600)).timestamp(),
        }
    }

    pub fn get_email(&self) -> &str {
        self.email.as_str()
    }
}
