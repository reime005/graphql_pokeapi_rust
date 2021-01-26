use pokerust::{Berry, FromId};

#[macro_use]
extern crate juniper;
extern crate serde_json;

use actix_web::{middleware, web, App, HttpServer};

use crate::handlers::setup;

mod handlers;
mod schemas;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let addr = std::env::var("ADDRESS").unwrap_or("127.0.0.1".to_owned());
  let port = std::env::var("PORT").unwrap_or("8080".to_owned());

  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .configure(setup)
      .default_service(web::to(|| async { "404" }))
  })
  .bind(format!("{}:{}", addr, port))?
  .run()
  .await
}
