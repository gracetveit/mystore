use crate::schema::products;
use crate::db_connection::establish_connection;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
  pub id: i32,
  pub name: String,
  pub stock: f64,
  pub price: Option<i32>
}

impl Product {
  pub fn find(id: &i32) -> Result<Product, diesel::result::Error> {
    let connection = establish_connection();

    products::table.find(id).first(&connection)
  }
}

#[derive(Insertable, Deserialize)]
#[table_name="products"]
pub struct NewProduct {
  pub name: Option<String>,
  pub stock: Option<f64>,
  pub price: Option<i32>
}

impl NewProduct {
  pub fn create(&self) -> Result<Product, diesel::result::Error> {
    use diesel::prelude::*;

    let connection = establish_connection();
    
    diesel::insert_into(products::table)
      .values(self)
      .get_result(&connection)
  }
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList {
  pub fn list() -> Self {
    use crate::schema::products::dsl::*;

    let connection = establish_connection();

    let result =
      products
        .limit(10)
        .load::<Product>(&connection)
        .expect("Error loading products");
    
    ProductList(result)
  }
}