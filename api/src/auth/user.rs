use crate::errors::ServiceError;
use actix_identity::Identity;
use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use arc_swap::ArcSwap;
use argonautica::{Hasher, Verifier};
use chrono::{DateTime, Utc};
use crossbeam::atomic::AtomicCell;
use deadpool_postgres::{Client, Pool};
use lazy_static::lazy_static;
use log::debug;
use rand::{thread_rng, Rng};
use tokio::fs;
use tokio::{fs::File, io::AsyncReadExt};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

// use tokio_pg_mapper::FromTokioPostgresRow;
// use tokio_pg_mapper_derive::PostgresMapper;

use futures::{
    executor::block_on,
    future::{ready, Ready},
};
use im::HashMap;

use postgres_query::FromSqlRow;
use serde::{Deserialize, Serialize};
use std::{
    cell::Cell,
    env, io,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::LocalKey,
};

//logged in

use crate::auth::token::Token;
use crate::auth::Claim;
use deadpool_postgres::tokio_postgres::Row;
use smallvec::SmallVec;

// use crate::{
//     claim::Claim, errors::ServiceError as Error, pgpool::PgPool, token::Token, user::User,
// };

pub const KEY_LENGTH: usize = 32;

type SecretKey = [u8; KEY_LENGTH];

thread_local! {
    static SECRET_KEY_CACHE: Cell<Option<SecretKey>> = Cell::new(None);
    static JWT_SECRET_CACHE: Cell<Option<SecretKey>> = Cell::new(None);
}

lazy_static! {
    pub static ref AUTHORIZED_USERS: AuthorizedUsers = AuthorizedUsers::new();
    pub static ref TRIGGER_DB_UPDATE: AuthTrigger = AuthTrigger::new();
    pub static ref SECRET_KEY: AuthSecret = AuthSecret::new(SECRET_KEY_CACHE);
    pub static ref JWT_SECRET: AuthSecret = AuthSecret::new(JWT_SECRET_CACHE);
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct LoggedUser {
    pub email: String,
}

impl<'a> From<Claim> for LoggedUser {
    fn from(claim: Claim) -> Self {
        Self {
            email: claim.get_email().into(),
        }
    }
}

impl From<User> for LoggedUser {
    fn from(user: User) -> Self {
        Self { email: user.email }
    }
}

fn _from_request(req: &HttpRequest, pl: &mut Payload) -> Result<LoggedUser, ServiceError> {
    if let Ok(s) = env::var("TESTENV") {
        if &s == "true" {
            return Ok(LoggedUser {
                email: "user@test".into(),
            });
        }
    }
    if let Some(identity) = block_on(Identity::from_request(req, pl))?.identity() {
        let user: LoggedUser = Token::decode_token(&identity.into())?.into();
        if AUTHORIZED_USERS.is_authorized(&user) {
            return Ok(user);
        } else {
            debug!("not authorized {:?}", user);
        }
    }
    Err(ServiceError::Unauthorized(String::from("Unauthorized")))
}

impl FromRequest for LoggedUser {
    type Error = ServiceError;
    type Future = Ready<Result<Self, ServiceError>>;
    type Config = ();

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        ready(_from_request(req, pl))
    }
}

#[derive(Clone, Debug, Copy)]
enum AuthStatus {
    Authorized(DateTime<Utc>),
    NotAuthorized,
}

#[derive(Debug, Default)]
pub struct AuthorizedUsers(ArcSwap<HashMap<LoggedUser, AuthStatus>>);

impl AuthorizedUsers {
    pub fn new() -> Self {
        Self(ArcSwap::new(Arc::new(HashMap::new())))
    }

    pub fn is_authorized(&self, user: &LoggedUser) -> bool {
        if let Some(AuthStatus::Authorized(last_time)) = self.0.load().get(user) {
            let current_time = Utc::now();
            if (current_time - *last_time).num_minutes() < 15 {
                return true;
            }
        }
        false
    }

    pub fn store_auth(&self, user: LoggedUser, is_auth: bool) -> Result<(), ServiceError> {
        let current_time = Utc::now();
        let status = if is_auth {
            AuthStatus::Authorized(current_time)
        } else {
            AuthStatus::NotAuthorized
        };
        let auth_map = Arc::new(self.0.load().update(user, status));
        self.0.store(auth_map);
        Ok(())
    }

    pub fn merge_users(&self, users: &[LoggedUser]) -> Result<(), ServiceError> {
        let mut auth_map = (*self.0.load().clone()).clone();
        let not_auth_users: Vec<_> = auth_map
            .keys()
            .cloned()
            .filter(|user| !users.contains(user))
            .collect();
        for user in not_auth_users {
            if !users.contains(&user) {
                auth_map.insert(user.clone(), AuthStatus::NotAuthorized);
            }
        }
        for user in users {
            auth_map.insert(user.clone(), AuthStatus::Authorized(Utc::now()));
        }
        self.0.store(Arc::new(auth_map));
        Ok(())
    }

    pub fn get_users(&self) -> Vec<LoggedUser> {
        self.0.load().keys().cloned().collect()
    }
}

#[derive(Debug, Default)]
pub struct AuthTrigger(AtomicBool);

impl AuthTrigger {
    pub fn new() -> Self {
        Self(AtomicBool::new(true))
    }

    pub fn check(&self) -> bool {
        self.0.compare_and_swap(true, false, Ordering::SeqCst)
    }

    pub fn set(&self) {
        self.0.store(true, Ordering::SeqCst)
    }
}

pub async fn fill_auth_from_db(db_pool: &Client) -> Result<(), ServiceError> {
    debug!("{:?}", *TRIGGER_DB_UPDATE);
    let users: Vec<LoggedUser> = if TRIGGER_DB_UPDATE.check() {
        User::get_authorized_users(&db_pool)
            .await?
            .into_iter()
            .map(|user| LoggedUser { email: user.email })
            .collect()
    } else {
        AUTHORIZED_USERS.get_users()
    };
    AUTHORIZED_USERS.merge_users(&users)?;
    debug!("{:?}", *AUTHORIZED_USERS);
    Ok(())
}

pub struct AuthSecret(
    AtomicCell<Option<SecretKey>>,
    LocalKey<Cell<Option<SecretKey>>>,
);

impl AuthSecret {
    pub fn new(cache: LocalKey<Cell<Option<SecretKey>>>) -> Self {
        Self(AtomicCell::new(None), cache)
    }

    pub fn get(&'static self) -> SecretKey {
        if let Some(key) = self.1.with(Cell::get) {
            key
        } else if let Some(key) = self.0.load() {
            self.1.with(|cache| cache.set(Some(key)));
            key
        } else {
            panic!("Attempting to use uninitialized secret key");
        }
    }

    pub fn set(&self, key: SecretKey) {
        self.0.store(Some(key));
    }

    pub async fn read_from_file(&self, p: &Path) -> Result<(), ServiceError> {
        if p.exists() {
            let mut secret = [0_u8; KEY_LENGTH];
            let mut f = File::open(p).await?;
            f.read_exact(&mut secret).await?;
            self.0.store(Some(secret));
            Ok(())
        } else {
            // Err(ServiceError::GenericError("Secret file {} doesn't exist", p.to_string_lossy()))
            Err(ServiceError::GenericError(String::from(format!(
                "Secret file {} doesn't exist",
                p.to_string_lossy()
            ))))
        }
    }
}

pub async fn update_secret(p: &Path) -> Result<(), ServiceError> {
    if p.exists() {
        Ok(())
    } else {
        create_secret(p).await
    }
}

pub async fn create_secret(p: &Path) -> Result<(), ServiceError> {
    fs::write(p, &get_random_key()).await?;
    Ok(())
}

pub fn get_random_key() -> SmallVec<SecretKey> {
    let mut rng = thread_rng();
    (0..KEY_LENGTH).map(|_| rng.gen::<u8>()).collect()
}

pub async fn get_secrets<T: AsRef<Path>>(
    secret_path: T,
    jwt_secret_path: T,
) -> Result<(), ServiceError> {
    SECRET_KEY.read_from_file(secret_path.as_ref()).await?;
    JWT_SECRET.read_from_file(jwt_secret_path.as_ref()).await
}

// logged in

// use crate::{app::CONFIG, errors::ServiceError as Error, pgpool::PgPool};

// pub fn hash_password(plain: &str) -> StackString {
//     // get the hashing cost from the env variable or use default
//     hash(plain, CONFIG.hash_rounds)
//         .expect("Password Hashing failed")
//         .into()
// }

#[derive(Serialize, Deserialize, PartialEq, Debug, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub email: String,
    // password here is always the hashed password
    password: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn hash_password(password: &str) -> Result<String, ServiceError> {
        Hasher::default()
            .with_password(password)
            .with_secret_key("0123".repeat(8))
            .hash()
            .map_err(|_| ServiceError::GenericError(String::from("Could not hash password")))
    }

    // TODO config secret key
    pub async fn verify(&self, hash: &str) -> Result<bool, ServiceError> {
        let password = &self.password;
        Verifier::default()
            .with_hash(hash)
            .with_password(password)
            .with_secret_key("0123".repeat(8)) //TODO config secret key
            .verify()
            .map_err(|_| {
                ServiceError::AuthenticationError(String::from("Could not verify password"))
            })
    }

    pub async fn from_details(email: &str, password: &str) -> Result<Self, ServiceError> {
        // let password = hash_password(password);
        // let password = self.hash_password(password);

        let password = User::hash_password(password).await?;

        // let password = hash_password()?;
        Ok(Self {
            email: email.into(),
            password,
            created_at: Utc::now(),
        })
    }

    pub async fn set_password(&mut self, password: &str) -> Result<(), ServiceError> {
        // self.password = self.hash_password()
        self.password = User::hash_password(password).await?;
        Ok(())
    }

    pub async fn verify_password(&self, password: &str) -> Result<bool, ServiceError> {
        // verify(password, &self.password)
        self.verify(password).await?;
        Ok(true)
    }

    pub async fn get_authorized_users(client: &Client) -> Result<Vec<Self>, ServiceError> {
        // let query = postgres_query::query!("SELECT * FROM users");

        let statement = client.prepare("SELECT * FROM users").await.unwrap();

        let content_list = client
            .query(&statement, &[])
            .await
            .expect("Error getting author lists")
            .iter()
            // .map(|row| Self::from_row(row).unwrap())
            .map(|row| Self::from_row_ref(&row).unwrap())
            .collect::<Vec<Self>>();

        Ok(content_list)
    }

    pub async fn get_number_users(client: &Client) -> Result<i64, ServiceError> {
        let query = client.prepare("SELECT count(*) FROM users").await.unwrap();
        let maybe_count = client.query_one(&query, &[]).await?.try_get(0)?;
        Ok(maybe_count)
    }

    pub async fn get_by_email(email: &str, client: &Client) -> Result<Option<Self>, ServiceError> {
        // let query =
        //     postgres_query::query!("SELECT * FROM users WHERE email = $email", email = email);

        let statement = client
            .prepare("SELECT * FROM users WHERE email = $email")
            .await
            .unwrap();

        let maybe_user = client
            .query_opt(&statement, &[&email])
            .await
            .expect("Error fetching content ")
            .map(|row| Self::from_row_ref(&row).unwrap());

        match maybe_user {
            Some(user) => Ok(Some(user)),
            None => Err(ServiceError::AuthenticationError(String::from(
                "Could not verify password",
            ))),
        }
    }

    pub async fn insert(&self, client: &Client) -> Result<(), ServiceError> {
        let query = client
            .prepare(
                "INSERT INTO users (email, password, created_at)
            VALUES ($email, $password, $created_at)",
            )
            .await
            .unwrap();

        client
            .query(&query, &[&self.email, &self.password])
            .await
            .unwrap();

        Ok(())
    }

    pub async fn update(&self, client: &Client) -> Result<(), ServiceError> {
        // let query = postgres_query::query!(
        //     "UPDATE users set password = $password WHERE email = $email",
        //     password = self.password,
        //     email = self.email,
        let query = client
            .prepare("UPDATE users set password = $password WHERE email = $email")
            .await
            .unwrap();
        let result = client
            .execute(&query, &[&self.password, &self.password])
            .await
            .expect("Error updating user");
        Ok(())

        // match result {
        //     ref updated if *updated == 1 => Ok(()),
        //     None => Err(ServiceError::PostgressError(String::from(
        //         "Could not verify password",
        //     ))),
        // }
    }

    pub async fn upsert(&self, db_pool: &Client) -> Result<(), ServiceError> {
        if Self::get_by_email(&self.email, db_pool).await?.is_some() {
            self.update(db_pool).await
        } else {
            self.insert(db_pool).await
        }
    }

    pub async fn delete(&self, pool: web::Data<Pool>) -> Result<(), ServiceError> {
        let query =
            postgres_query::query!("DELETE FROM users WHERE email = $email", email = self.email);
        pool.get()
            .await?
            .execute(query.sql(), query.parameters())
            .await?;
        Ok(())
    }
}
//
// #[cfg(test)]
// mod tests {
//
//     use crate::errors::ServiceError;
//     // use crate::{
//     //     app::{get_random_string, CONFIG},
//     //     // pgpool::PgPool,
//     //     // user::User,
//     // };
//     use crate::get_random_string;
//     use deadpool_postgres::Client;
//
//     #[tokio::test]
//     async fn test_create_delete_user() -> Result<(), ServiceError> {
//         // let pool = PgPool::new(&CONFIG.database_url);
//
//         let client: Client = data.get().await.expect("Error connecting to the database");
//
//         let email = format!("{}@localhost", get_random_string(32));
//
//         assert_eq!(User::get_by_email(&email, &client).await?, None);
//
//         let password = get_random_string(32);
//         let user = User::from_details(&email, &password);
//
//         user.insert(&pool).await?;
//         let mut db_user = User::get_by_email(&email, &pool).await?.unwrap();
//         println!("{:?}", db_user);
//         assert!(db_user.verify_password(&password)?);
//
//         let password = get_random_string(32);
//         db_user.set_password(&password);
//         db_user.upsert(&pool).await?;
//
//         let db_user = User::get_by_email(&email, &pool).await?.unwrap();
//         println!("{:?}", db_user);
//         assert!(db_user.verify_password(&password)?);
//
//         db_user.delete(&pool).await?;
//         assert_eq!(User::get_by_email(&email, &pool).await?, None);
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn test_get_authorized_users_get_number_users() -> Result<(), ServiceError> {
//         let pool = PgPool::new(&CONFIG.database_url);
//         let count = User::get_number_users(&pool).await? as usize;
//         let users = User::get_authorized_users(&pool).await?;
//         println!("{:?}", users);
//         assert_eq!(count, users.len());
//         Ok(())
//     }
// }
