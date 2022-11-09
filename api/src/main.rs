use actix_web::{web, App, HttpServer};
pub mod category;

pub mod configs;

use deadpool_postgres::Runtime;
use dotenv::dotenv;

use crate::configs::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::from_env().unwrap();
    let bind_addr = format!("{}:{}", config.srv_cnf.host, config.srv_cnf.port);
    let pool = config
        .pg
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .unwrap();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/categories").configure(category::init_routes))
    })
    .bind(bind_addr)?
    .bind_uds("/tmp/actix-uds.socket")?
    .run();

    server.await
}
