use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default, Clone, Deserialize)]
pub struct User {
    pub link: String,
    pub password: String,
    pub email: String,
}
#[derive(Serialize, Debug, Default, Clone, Deserialize)]
pub struct Link {
    pub linkname: String,
}
