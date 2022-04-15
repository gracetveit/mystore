use actix_web::{ HttpServer, App, web, HttpResponse, Responder};
// extern crate serde;
// extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

pub mod handlers;
pub mod schema;
pub mod db_connection;
pub mod models;

async fn index() -> impl Responder {
    HttpResponse::Ok().json("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)).service(handlers::products::index))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
