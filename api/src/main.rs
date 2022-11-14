use actix_cors::Cors;
use actix_web::cookie::Key;
use actix_web::main;
// use auth::update_secret;
use time::Duration;

use std::collections::HashMap;

// pub mod auth;
pub mod configs;
pub mod users;

// pub mod content;
pub mod errors;
// pub mod mail;

// pub mod media;
// pub mod pages;
// pub mod templ;

// For google ID

// use crate::auth::{
//     auth_url, callback, change_password, change_password_user, index_html, login_html, main_css,
//     main_js, register_email, register_html, register_user,
// };

// use crate::auth::{
//     change_password, change_password_user, get_me, get_secrets, index_one, login, login_html,
//     logout, main_css, main_js, register_email, register_html, register_user, update_secret,
// };

// use crate::auth::{change_password_user, register_user};
use crate::configs::{SecConfig, KEY_LENGTH};

use crate::errors::ServiceError;
use actix_files as fs;
// use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::middleware::{Compress, Logger};
use actix_web::{http, http::header, middleware, web, App, HttpResponse, HttpServer, Result};
use argonautica::{Hasher, Verifier};
use deadpool_postgres::Runtime;
use dotenv::dotenv;
use env_logger;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::io;
use tokio_postgres::NoTls;

lazy_static! {
    pub static ref CONFIG: SecConfig = SecConfig::init_config().expect("Failed to init config");
}

pub fn get_random_string(n: usize) -> String {
    let mut rng = thread_rng();
    (0..)
        .filter_map(|_| {
            let c: char = (rng.gen::<u8>() & 0x7f).into();
            match c {
                ' '..='~' => Some(c),
                _ => None,
            }
        })
        .take(n)
        .collect()
}

// async fn update_secrets() -> Result<(), ServiceError> {
//     update_secret(&CONFIG.secret_path).await?;
//     update_secret(&CONFIG.jwt_secret_path).await
// }

//
// async fn update_secreto() -> Result<(), ServiceError> {
//     update_secret(&CONFIG.secret_path).await?;
//     update_secret(&CONFIG.jwt_secret_path).await
// }

// pub async fn start_app() -> Result<(), ServiceError> {
//     update_secrets().await?;
//     get_secrets(&CONFIG.secret_path, &CONFIG.jwt_secret_path).await?;
//     run_app(CONFIG.port, SECRET_KEY.get(), CONFIG.domain.clone()).await
// }

//
#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    // update_secrets().await;

    let config = configs::Config::from_env().unwrap();
    let domain = config.srv_cnf.host.clone();
    // let pool = config.pg.create_pool(NoTls).unwrap();
    let pool = config
        .pg
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .unwrap();
    let bind_addr = format!("{}:{}", config.srv_cnf.host, config.srv_cnf.port);

    println!(
        "Starting server at http://{}:{}",
        config.srv_cnf.host, config.srv_cnf.port
    );

    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    // hash

    // end of hash

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    // .supports_credentials()
                    // .send_wildcard()
                    // .allowed_headers(vec![
                    //     http::header::ORIGIN,
                    //     header::AUTHORIZATION,
                    //     header::ACCEPT,
                    //     header::CONTENT_TYPE,
                    //     header::REFERER,
                    //     header::ALLOW
                    // ])
                    .expose_any_header()
                    .allow_any_method()
                    .allow_any_header()
                    .allow_any_origin()
                    // .allowed_origin("http://127.0.0.1:8081")
                    // .allowed_origin_fn(|origin, _req_head| {
                    //     origin.as_bytes().ends_with(b":8081")
                    // })
                    .max_age(3600),
            )
            // TODO .wrap(Compress::default()) Check if compression is important for API
            // .wrap(middleware::NormalizePath::default())
            .wrap(Logger::default())
            .wrap(Logger::new(
                "%a %r %s %b %% %U %{FOO}i %{FOO}o  %{User-Agent}i",
            ))
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(&private_key)
            //         .name("auth")
            //         .path("/")
            //         .domain(&domain)
            //         .max_age(3600) //   TODO Parametize into config.rs
            //         .secure(false),
            // ))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&private_key))
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(1)))
                    .cookie_name("auth".to_owned())
                    .cookie_secure(false)
                    .cookie_domain(Some(domain.clone()))
                    .cookie_path("/".to_owned())
                    .build(),
            )
            .service(
                web::scope("/api")
                    // .service(
                    //     web::resource("/auth")
                    //         .route(web::post().to(login))
                    //         .route(web::delete().to(logout))
                    //         .route(web::get().to(get_me)),
                    // )
                    //
                    //
                    // .service(web::resource("/invitation").route(web::post().to(register_email)))
                    .service(web::scope("/categories").configure(users::init_routes)), // Take into account the comma of the last element
                                                                                       // Registration
                                                                                       // .service(
                                                                                       //     web::resource("/register/{invitation_id}")
                                                                                       //         .route(web::post().to(register_user)),
                                                                                       // )
                                                                                       // .service(
                                                                                       //     web::resource("/password_change")
                                                                                       //         .route(web::post().to(change_password_user)),
                                                                                       // ),
                                                                                       // .service(web::resource("/auth_url").route(web::post().to(auth_url)))
                                                                                       // .service(web::resource("/callback").route(web::get().to(callback))), // .service(web::resource("/status").route(web::get().to(status))),
            )
        // .service(
        //     //         TODO check to avoid duplicate email activation. query from user table if email exists.
        //     web::scope("/auth")
        //         .service(web::resource("/index.html").route(web::get().to(index_one)))
        //         .service(web::resource("/main.css").route(web::get().to(main_css)))
        //         .service(web::resource("/main.js").route(web::get().to(main_js)))
        //         .service(web::resource("/register.html").route(web::get().to(register_html)))
        //         .service(web::resource("/login.html").route(web::get().to(login_html)))
        //         .service(
        //             web::resource("/change_password.html")
        //                 .route(web::get().to(change_password)),
        //         ),
        // )
        //

        // .service(
        //     web::resource("/")
        //         .name("home")
        //         .route(web::get().to(pages::index)),
        // )

        // .service(web::resource("/favicon.ico").route(web::get().to(|| HttpResponse::Ok())))

        // .service(
        //     web::resource("/{content}")
        //         .name("content")
        //         .route(web::get().to(pages::index)),
        // )

        // .service(web::resource("/").route(web::get().to(index)))
        // .service(web::resource("/admin/*").route(web::get().to(index)))
        // .service(web::scope("/content").configure(content::init_routes))
        // .service(fs::Files::new("/", "./templates").show_files_listing())
    })
    .bind(bind_addr)?
    .run()
    .await
}
