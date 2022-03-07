use entity::fruits;
// use entity::suppliers;
use entity::sea_orm;
use migration::{Migrator, MigratorTrait};
use chrono::Utc;
use sea_orm::{entity::Set, prelude::*};
use anyhow::Result;
use std::env;

#[async_std::main]
async fn main() -> Result<()>{
  // get env vars
  dotenv::dotenv().ok();
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
  let host = env::var("HOST").expect("HOST is not set in .env file");
  let port = env::var("PORT").expect("PORT is not set in .env file");
  let server_url = format!("{}:{}", host, port);
  println!("Starting server at {}", server_url);

  let conn = sea_orm::Database::connect(&db_url).await.unwrap();
  Migrator::up(&conn, None).await.unwrap();
  println!("Database connected!");

  let fruit_01 = fruits::ActiveModel {
    name: Set("Apple".to_owned()),
    datetime_utc: Set(Utc::now().naive_utc()),
    unit_price: Set(2),
    sku: Set("FM2022AKB40".to_owned()),
    ..Default::default()
  };
  let fruit_insert_operation = fruits::Entity::insert(fruit_01).exec(&conn).await;
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