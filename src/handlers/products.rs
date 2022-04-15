use actix_web::{get, Responder, HttpResponse};
use crate::models::product::ProductList;

#[get("/products")]
pub async fn index() -> impl Responder {
  HttpResponse::Ok().json(ProductList::list())
}