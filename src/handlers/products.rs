use actix_web::{get, post, HttpResponse, web, };
use crate::models::product::{ProductList, NewProduct, };

#[get("/products")]
pub async fn index() -> HttpResponse {
  HttpResponse::Ok().json(ProductList::list())
}

#[post("/products")]
pub async fn create(new_product: web::Json<NewProduct>) -> HttpResponse {
  let result = new_product
    .create();

  match result {
    Ok(product) => HttpResponse::Ok().json(product),
    Err(e) => HttpResponse::InternalServerError().json(e.to_string())
  }
}