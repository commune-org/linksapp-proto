// use actix_session::Session;
// use actix_web::{error::BlockingError, http::header::LOCATION, web, HttpResponse};
// use deadpool_postgres::{Client, Pool};
// // use io::ErrorKind::NotFound;
// use crate::errors::ServiceError;
// use serde::Deserialize;
// use tokio_pg_mapper::FromTokioPostgresRow;
// use uuid::Uuid;
//
// use crate::auth::{
//     hash_password, is_signed_in, set_current_user, to_home, Confirmation, SessionUser, User,
// };
//
// // // use yarte::Template;
// //
// // use crate::errors::ServiceError;
// // use deadpool_postgres::Pool;
// // use std::io;
// //
// // #[derive(Debug, Deserialize)]
// // pub struct PasswordData {
// //     pub password: String,
// // }
// //
// // pub async fn create_account(
// //     mut session: Session,
// //     path_id: web::Path<String>,
// //     data: web::Json<PasswordData>,
// //     pool: web::Data<Pool>,
// // ) -> Result<HttpResponse, ServiceError> {
// //     if is_signed_in(&session) {
// //         return Ok(HttpResponse::BadRequest().finish());
// //     }
// //
// //     let result =
// //         web::block(move || create_user(&path_id.into_inner(), &data.into_inner().password, &pool))
// //             .await;
// //
// //     match result {
// //         Ok(user) => {
// //             set_current_user(&mut session, &user);
// //
// //             Ok(HttpResponse::Created().json(&user))
// //         }
// //         Err(err) => match err {
// //             BlockingError::Error(auth_error) => Err(auth_error),
// //             BlockingError::Canceled => Err(ServiceError::GenericError(String::from(
// //                 "Could not complete the process",
// //             ))),
// //         },
// //     }
// // }
// //
// // pub async fn show_password_form(
// //     session: Session,
// //     path_id: web::Path<String>,
// //     pool: web::Data<Pool>,
// // ) -> Result<HttpResponse, ServiceError> {
// //     if is_signed_in(&session) {
// //         Ok(to_home())
// //     } else {
// //         let id_str = path_id.into_inner();
// //
// //         match get_invitation(&id_str, &pool) {
// //             Ok(Confirmation { email, .. }) => {
// //                 let t = Password {
// //                     path_id: id_str,
// //                     email,
// //                     error: None,
// //                 };
// //
// //                 Ok(HttpResponse::Ok()
// //                     .content_type("text/html; charset=utf-8")
// //                     .body(t.call().unwrap()))
// //             }
// //             Err(_) => Ok(HttpResponse::MovedPermanently()
// //                 .header(LOCATION, "/register")
// //                 .finish()),
// //         }
// //     }
// // }
// //
// // pub async fn create_account_for_browser(
// //     path_id: web::Path<String>,
// //     data: web::Form<PasswordData>,
// //     mut session: Session,
// //     pool: web::Data<Pool>,
// // ) -> Result<HttpResponse, ServiceError> {
// //     let id_str = path_id.into_inner();
// //     let id_str2 = String::from(id_str.as_str());
// //     let result = web::block(move || create_user(&id_str, &data.into_inner().password, &pool)).await;
// //
// //     match result {
// //         Ok(user) => {
// //             set_current_user(&mut session, &user);
// //
// //             Ok(to_home())
// //         }
// //         Err(_) => {
// //             let t = Password {
// //                 path_id: id_str2,
// //                 email: String::from("unknown@email.com"),
// //                 error: Some(String::from("Invalid/expired confirmation id")),
// //             };
// //
// //             Ok(HttpResponse::Ok()
// //                 .content_type("text/html; charset=utf-8")
// //                 .body(t.call().unwrap()))
// //         }
// //     }
// // }
// //
// // fn get_invitation(path_id: &str, pool: &web::Data<Pool>) -> Result<Confirmation, ServiceError> {
// //     let path_uuid = Uuid::parse_str(path_id)?;
// //
// //     if let Ok(record) = confirmations
// //         .find(path_uuid)
// //         .get_result::<Confirmation>(&pool.get().unwrap())
// //     {
// //         Ok(record)
// //     } else {
// //         Err(ServiceError::AuthenticationError(String::from(
// //             "Invalid confirmation",
// //         )))
// //     }
// // }
// //
// pub async fn add_user() {}
// // pub async fn add_user(client: &Client, path_id: &str, password: &str) -> User {
// //     let path_uuid = Uuid::parse_str(path_id);
// //
// //     let statement = client
// //         .prepare("select email, expires_at from public.confirmation where id = $1")
// //         .await
// //         .unwrap();
// //
// //     // let maybe_user = client.query_opt(&statement, &[&path_uuid]).await?;
// //     let user = match client.query_opt(&statement, &[&path_uuid]) {
// //         Some(row) => SessionUser {
// //             id: row.get(0),
// //             email: row.get(1),
// //         },
// //         None => Err(ServiceError::BadId),
// //         _ => {}
// //     };
// //
// //     let insert_statement = client
// //         .prepare("INSERT INTO public.user(email, hash_password) VALUES ($0, $1) RETURNING email")
// //         .await
// //         .unwrap();
// //
// //     let password: String = hash_password(password)?;
// //
// //     // let email: String = maybe_user.get(0)?;
// //     let email: String = user.email;
// //
// //     let rows = client
// //         .query(&insert_statement, &[&email, &password])
// //         .await?;
// //
// //     // let users: Vec<_> = rows
// //     //     .into_iter()
// //     //     .map(|row| SessionUser {
// //     //         id: row.get(0),
// //     //         email: row.get(1),
// //     //     })
// //     //     .collect();
// //
// //     let result = client
// //         .query(&insert_statement, &[&email, &password])
// //         .await
// //         .expect("Error creating content")
// //         .iter()
// //         .map(|row| User::from_row_ref(row).unwrap())
// //         .collect::<Vec<User>>()
// //         .pop()
// //         .ok_or(ServiceError::ProcessError);
// //
// //     match result {
// //         Ok(content) => content,
// //         Err(err) => ServiceError::BadId,
// //     }
// // }
//
// // // for row in client.query(
// // //     "select id, expires_at from public.confirmation where id = $1",
// // //     &[&path_uuid],
// // // )? {
// // //     let id: i32 = row.get(0);
// // //     let expires_at: chrono::NaiveDateTime = row.get(1);
// // //
// // //     println!("Found person: {}", id);
// // //
// // //     if expires_at > chrono::Local::now().naive_local() {
// // //         let password: String = hash_password(password)?;
// // //         client.execute(
// // //             "INSERT INTO articles (title, body, published_at, author_id) VALUES ($1, $2, $3, $4)",
// // //             &[&title, &body, &cur_time, &id],
// // //         ).await.expect("error inserting user").pop().ok_or(io::Error::new(io::ErrorKind::Other, "Error Adding user"));
// // //     }
// // //
// // //     return Ok(row);
// // // }
// //
// // // async fn event_list(pool: &Pool) -> Result<Vec<Event>, PoolError> {
// // //     let client: Client = pool.get().await?;
// // //     let stmt = client.prepare("SELECT id, title FROM event").await?;
// // //     let rows = client.query(&stmt, &[]).await?;
// // //     Ok(rows
// // //         .into_iter()
// // //         .map(|row| Event {
// // //             id: row.get(0),
// // //             title: row.get(1),
// // //         })
// // //         .collect())
// // // }
// //
// // // // TODO change id_content: i32 to uuid or string
// // // pub async fn user_uuid(client: &Client, id_content: i32) -> Result<Confirmation, io::Error> {
// // //     let statement = client
// // //         .prepare("select * from public.confirmation where id = $1")
// // //         .await
// // //         .unwrap();
// // //
// // //     let maybe_content = client
// // //         .query_opt(&statement, &[&id_content])
// // //         .await
// // //         .expect("Error fetching content ")
// // //         .map(|row| Content::from_row_ref(&row).unwrap());
// // //
// // //     match maybe_content {
// // //         Some(content) => Ok(content),
// // //         None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found")),
// // //     }
// // // }
// //
// // // pub async fn get_news_by_id(id: &String) -> Option<Confirmation> {
// // //     let client = connect().await.unwrap();
// // //     let rows = &client
// // //         .query(
// // //             "SELECT id::text,url,'desc' FROM news where id::text=$1",
// // //             &[&id],
// // //         )
// // //         .await
// // //         .unwrap();
// // //     let row = rows.get(0).unwrap();
// // //     let news = News {
// // //         id: row.get(0),
// // //         desc: row.get(2),
// // //         url: row.get(1),
// // //     };
// // //     return Some(news);
// // // }
// //
// // // pub async fn add_user(
// // //     client: &Client,
// // //     path_id: &str,
// // //     password: &str,
// // //     pool: &web::Data<Pool>,
// // // ) -> Result<SessionUser, ServiceError> {
// // //     let path_uuid = Uuid::parse_str(path_id)?;
// // //     let conn = &pool.get().unwrap();
// // //     let statement = client
// // //         .prepare(
// // //             "INSERT INTO public.users (email, hash_password) VALUES ($0, $1)
// // //     RETURNING id, email",
// // //         )
// // //         .await
// // //         .unwrap();
// // //
// // //     let result = user_uuid(&client, path_id.parse().unwrap()).await;
// // //
// // //     client
// // //         .query(&statement, &[&selfobj.email, &selfobj.hash_password])
// // //         .await
// // //         .expect("Error Adding User")
// // //         .iter()
// // //         .map(|row| User::from_row_ref(row).unwrap())
// // //         .collect::<Vec<User>>()
// // //         .pop()
// // //         .ok_or(io::Error::new(
// // //             io::ErrorKind::Other,
// // //             "Error creating content tables",
// // //         ));
// // //
// // //     match result {
// // //         Some(object) => Ok(object),
// // //         // Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
// // //         Err(ref e) if e.kind() == NotFound => io::ErrorKind::NotFound,
// // //         // None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found")),
// // //         // Err(_) => HttpResponse::InternalServerError().into(),
// // //         _ => {}
// // //     }
// // //
// // //
// // //     for row: Row in client.query("SELECT * FROM users"), &[])? {
// // //     let id: i32 = row.get(0);
// // //     let title: &str = "A Great Article!";
// // //     let body: &str = "You should share this with friends.";
// // //     let cur_time: DateTime<Utc> = Utc::now();
// // //     client.execute(
// // //     "INSERT INTO articles (title, body, published_at, author_id) VALUES ($1, $2, $3, $4)",
// // //     &[&title, &body, &cur_time, &id]
// // //     )?;
// // //     }
// // //
// // //     // confirmations
// // //     //     .filter(id.eq(path_uuid))
// // //     //     .load::<Confirmation>(conn)
// // //     //     .map_err(|_db_error| ServiceError::NotFound(String::from("Confirmation not found")))
// // //     //     .and_then(|mut result| {
// // //     //         if let Some(confirmation) = result.pop() {
// // //     //             if confirmation.expires_at > chrono::Local::now().naive_local() {
// // //     //                 // confirmation has not expired
// // //     //                 let password: String = hash_password(password)?;
// // //     //                 let user: User = diesel::insert_into(users)
// // //     //                     .values(&User::from(confirmation.email, password))
// // //     //                     .get_result(conn)?;
// // //     //
// // //     //                 return Ok(user.into());
// // //     //             }
// // //     //         }
// // //     //         Err(ServiceError::AuthenticationError(String::from(
// // //     //             "Invalid confirmation",
// // //     //         )))
// // //     //     })
// // // }
// // // fn create_user(
// // //     path_id: &str,
// // //     password: &str,
// // //     pool: &web::Data<Pool>,
// // // ) -> Result<SessionUser, ServiceError> {
// // //     let path_uuid = Uuid::parse_str(path_id)?;
// // //     let conn = &pool.get().unwrap();
// // //
// // //     confirmation
// // //         .filter(id.eq(path_uuid))
// // //         .load::<Confirmation>(conn)
// // //         .map_err(|_db_error| ServiceError::NotFound(String::from("Confirmation not found")))
// // //         .and_then(|mut result| {
// // //             if let Some(confirmation) = result.pop() {
// // //                 if confirmation.expires_at > chrono::Local::now().naive_local() {
// // // // confirmation has not expired
// // //                     let password: String = hash_password(password)?;
// // //                     let user: User = diesel::insert_into(users)
// // //                         .values(&User::from(confirmation.email, password))
// // //                         .get_result(conn)?;
// // //
// // //                     return Ok(user.into());
// // //                 }
// // //             }
// // //
// // //             Err(ServiceError::AuthenticationError(String::from(
// // //                 "Invalid confirmation",
// // //             )))
// // //         })
// // // }
