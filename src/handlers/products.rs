use actix_web::{get, post, delete, HttpResponse, web, };
use crate::models::product::{ProductList, NewProduct, Product};


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

#[get("/products/{id}")]
pub async fn show(id: web::Path<i32>) -> HttpResponse {
  let result = Product::find(&id);
  match result {
    Ok(product) => HttpResponse::Ok().json(product),
    Err(e) => HttpResponse::InternalServerError().json(e.to_string())
  }
}

#[delete("/products/{id}")]
pub async fn delete(id: web::Path<i32>) -> HttpResponse {
  let result = Product::destroy(&id);
  match result {
    Ok(_) => HttpResponse::Ok().json(()),
    Err(e) => HttpResponse::InternalServerError().json(e.to_string())
  }
}