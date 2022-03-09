use entity::fruits;
// use entity::suppliers;
use entity::sea_orm;
use migration::{Migrator, MigratorTrait};
// use chrono::Utc;
use sea_orm::{entity::Set, prelude::*};
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use serde_json::to_string;
use dotenv::{dotenv, var};

#[derive(Debug, Clone)]
struct AppState {
  conn: DatabaseConnection,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
  let conn = &data.conn;
  let fruits_table_rows: Vec<fruits::Model> = fruits::Entity::find()
    .all(conn)
    .await
    .unwrap();
  let body = to_string(&fruits_table_rows).unwrap();

  HttpResponse::Ok()
    .content_type("application/json")
    .body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // get env vars
  dotenv().ok();
  let db_url = var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
  let host = var("HOST").expect("HOST is not set in .env file");
  let port = var("PORT").expect("PORT is not set in .env file");
  let server_url = format!("{}:{}", host, port);
  println!("Starting server at {}", server_url);

  let conn = sea_orm::Database::connect(&db_url).await.unwrap();
  Migrator::up(&conn, None).await.unwrap();
  println!("Database connected!");

  let state = web::Data::new(AppState { conn });
  HttpServer::new(move || App::new()
    .app_data(state.clone())
    .wrap(middleware::Compress::default())
    .service(index))
    .bind(&server_url)?
    .run()
    .await?;

  Ok(())
}