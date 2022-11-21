use crate::users::db;
use crate::users::models::CreateUsers;
use std::io;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use io::ErrorKind::NotFound;

#[get("/")]
pub async fn users(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::users_list(&client).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/")]
pub async fn add_users(
    local_object: web::Json<CreateUsers>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::users_add(&client, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
        // Err(_) => HttpResponse::InternalServerError().into(),
    }
}
// //error retrieving column count: error deserializing column 0: cannot convert between the Rust type `core::option::Option<i8>` and the Postgres type `int8`',
// #[get("/{id}/")]
// pub async fn get_count(id_users: web::Path<(String,)>, db_pool: web::Data<Pool>) -> impl Responder {
//     let client: Client = db_pool
//         .get()
//         .await
//         .expect("Error connecting to the database");

//     let result = db::user_count(&client, id_users.0.clone()).await;

//     match result {
//         Ok(object) => HttpResponse::Ok().json(object),
//         // Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
//         Err(e) => HttpResponse::NotFound().json(e.to_string()),
//         //Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }

#[get("/{id}")]
pub async fn get_link(id_users: web::Path<(String,)>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::user_link(&client, id_users.0.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        // Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
        //Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[delete("/{id}")]
pub async fn delete_users(users_id: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::users_delete(&client, users_id.0).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[patch("/{id}")]
pub async fn update_users(
    id_users: web::Path<(i32,)>,
    local_object: web::Json<CreateUsers>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::users_update(&client, id_users.0, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(users);
    cfg.service(add_users);
    cfg.service(update_users);
    cfg.service(get_link);
    // cfg.service(get_count);
    cfg.service(delete_users);
}

// #[delete("/{id}")]
// pub async fn delete_author(id_author: web::Path<(i32,)>,  db_pool: web::Data<Pool>) -> impl Responder {
//     let res = format!("{:?},", id_author.0);
//     println!("{:#?}", res);
//     res
// }
