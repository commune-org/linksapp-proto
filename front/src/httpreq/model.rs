use serde::{Deserialize, Serialize};

//#[derive(Serialize, Debug, Default, Clone, Deserialize)]
#[derive(Debug)]
#[perseus::make_rx(UserRx)]
pub struct User {
    pub link: String,
    pub password: String,
    pub email: String,
}

//#[derive(Serialize, Debug, Default, Clone, Deserialize)]
#[derive(Debug)]
#[perseus::make_rx(LinkRx)]
pub struct Link {
    pub linkname: String,
}
