use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct User {
    pub link: String,
    pub password: String,
}
