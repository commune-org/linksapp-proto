use serde::{Deserialize, Serialize};

// #[derive(Debug)]
// #[perseus::make_rx(UserRx)]
#[derive(Serialize, Debug, Default, Clone, Deserialize)]
pub struct User {
    pub link: String,
    pub password: String,
    pub email: String,
}

//#[derive(Debug)]
//#[perseus::make_rx(LinkRx)]
#[derive(Serialize, Debug, PartialEq, Eq, Default, Clone, Deserialize)]
pub struct Link {
    pub linkname: String,
}
