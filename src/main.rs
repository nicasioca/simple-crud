mod db;
use db::fruits_table::prelude::*;
use db::suppliers_table::prelude::*;
use chrono::Utc;
use sea_orm::{entity::Set, prelude::*, ConnectionTrait, Database, Schema};
use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()>{
  let env_database_url = include_str!("../.env").trim();
  let split_url: Vec<&str> = env_database_url.split("=").collect();
  let database_url = split_url[1];
  let db = Database::connect(database_url).await?;
  println!("Database connected!");

  let builder = db.get_database_backend();
  let schema = Schema::new(builder);
  let create_table_op = db
      .execute(builder.build(&schema.create_table_from_entity(Fruits)))
      .await;
  println!(
      "`CREATE TABLE fruits` {:?}",
      match create_table_op {
          Ok(_) => "Operation Successful".to_owned(),
          Err(e) => format!("Unsuccessful - Error {:?}", e),
      }
  );

  let fruit_01 = FruitsActiveModel {
    name: Set("Apple".to_owned()),
    datetime_utc: Set(Utc::now().naive_utc()),
    unit_price: Set(2),
    sku: Set("FM2022AKB40".to_owned()),
    ..Default::default()
  };
  let fruit_insert_operation = Fruits::insert(fruit_01).exec(&db).await;
  println!("INSERTED ONE: {:?}", fruit_insert_operation?);

  Ok(())
}


// use actix_web::web::{Either, Json, Form};
// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// #[derive(Deserialize)]
// struct Register {
//     username: String,
//     country: String,
// }

// // register form is JSON
// async fn register(form: web::Json<Register>) -> impl Responder {
//     format!("Hello {} from {}!", form.username, form.country)
// }

// // register form can be either JSON or URL-encoded
// async fn register(form: Either<Json<Register>, Form<Register>>) -> impl Responder {
//   let Register { username, country } = form.into_inner();
//   format!("Hello {username} from {country}!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }