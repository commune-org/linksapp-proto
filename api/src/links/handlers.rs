use crate::links::db;
use crate::links::Link;
use std::io;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use io::ErrorKind::NotFound;

#[get("/")]
pub async fn links_fetch(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::links_list(&client).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/")]
pub async fn add_link(local_object: web::Json<Link>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::link_add(&client, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
        // Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/{id}/")]
pub async fn get_count(id_users: web::Path<(String,)>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::link_count(&client, id_users.0.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        // Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
        //Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/{id}")]
pub async fn get_link(id_users: web::Path<(String,)>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::link_id(&client, id_users.0.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        // Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
        //Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[delete("/{id}")]
pub async fn delete_link(users_id: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> impl Responder {
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
pub async fn update_link(
    id_users: web::Path<(i32,)>,
    local_object: web::Json<Link>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::link_update(&client, id_users.0, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(links_fetch);
    cfg.service(add_link);
    cfg.service(update_link);
    cfg.service(get_link);
    cfg.service(get_count);
    cfg.service(delete_link);
}

// #[delete("/{id}")]
// pub async fn delete_author(id_author: web::Path<(i32,)>,  db_pool: web::Data<Pool>) -> impl Responder {
//     let res = format!("{:?},", id_author.0);
//     println!("{:#?}", res);
//     res
// }
