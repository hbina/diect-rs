#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod duration;
mod error;
mod persistent;
mod schema;
mod transient;

use crate::error::ApiError;
use crate::transient::transient_value::TransientDictionary;
use actix_web::{get, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

#[get("/")]
async fn index() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().finish())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    db::init();

    let data = TransientDictionary::create_storage();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(persistent::init_routes)
            .configure(transient::init_routes)
            .service(index)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}
