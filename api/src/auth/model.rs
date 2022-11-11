use chrono::{DateTime, Duration, NaiveDateTime, Utc};

use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

// #[derive(Deserialize)]
// pub struct CreateInvitation {
//     pub email: StackString,
// }
//

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: Uuid,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}

impl From<User> for SessionUser {
    fn from(User { email, id, .. }: User) -> Self {
        SessionUser { email, id }
    }
}

impl User {
    pub fn from<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
        User {
            id: Uuid::new_v4(),
            email: email.into(),
            hash: pwd.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
