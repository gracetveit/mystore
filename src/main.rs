use actix_web::{ HttpServer, App, web, HttpResponse, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().json("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
