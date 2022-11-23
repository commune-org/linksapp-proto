use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
extern crate chrono;
use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};

//To be added based on special query

#[derive(Serialize, Clone, Deserialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct Link {
    pub linkname: String,
}
