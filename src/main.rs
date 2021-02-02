use pokerust::{Berry, FromId};

#[macro_use]
extern crate juniper;
extern crate serde_json;

use actix_cors::Cors;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{middleware, web, App, HttpServer};
use std::time::Duration;

use crate::handlers::setup;

mod handlers;
mod schemas;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let addr = std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_owned());
    let port = std::env::var("PORT").unwrap_or("8080".to_owned());

    HttpServer::new(move || {
        let store = MemoryStore::new();

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                    .with_interval(Duration::from_secs(60))
                    .with_max_requests(100),
            )
            .wrap(middleware::Logger::default())
            .configure(setup)
            .default_service(web::to(|| async { "404" }))
    })
    .bind(format!("{}:{}", addr, port))?
    .run()
    .await
}
