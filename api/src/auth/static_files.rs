use crate::errors::ServiceError;
use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse};
use std::path::PathBuf;

// pub fn index_html() -> HttpResponse {
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(include_str!("templates/auth/index.html"))
// }

pub async fn index_one(_req: HttpRequest) -> Result<NamedFile, ServiceError> {
    let path: PathBuf = "./templates/auth/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

// async fn index_one(_req: HttpRequest) -> Result<NamedFile> {
//     let path: PathBuf = "./static/index.html".parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }

pub fn main_css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(include_str!("../../templates/auth/main.css"))
}

pub fn register_html() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../templates/auth/register.html"))
}

pub fn main_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript; charset=utf-8")
        .body(include_str!("../../templates/auth/main.js"))
}

pub fn login_html() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../templates/auth/login.html"))
}

pub fn change_password() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../templates/auth/change_password.html"))
}
