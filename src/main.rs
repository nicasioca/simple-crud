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

fn db_setup() -> (String, String) {
  dotenv().ok();
  let db_url = var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
  let host = var("HOST").expect("HOST is not set in .env file");
  let port = var("PORT").expect("PORT is not set in .env file");
  let server_url = format!("{}:{}", host, port);
  (db_url, server_url)
}

#[get("/")]
async fn show_fruits(data: web::Data<AppState>) -> HttpResponse {
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

#[get("/{id}")]
async fn fruit_detail(path: web::Path<i32>, data: web::Data<AppState>) -> HttpResponse {
  let id = path.into_inner();
  let conn = &data.conn;
  let fruits_table_rows : Option<fruits::Model> = fruits::Entity::find_by_id(id)
    .one(conn)
    .await
    .unwrap();
  let body = to_string(&fruits_table_rows).unwrap();

  HttpResponse::Ok()
    .content_type("application/json")
    .body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let (db_url, server_url) = db_setup();
  println!("Starting server at {}", server_url);

  let conn = sea_orm::Database::connect(&db_url).await.unwrap();
  Migrator::up(&conn, None).await.unwrap();
  println!("Database connected!");

  let state = web::Data::new(AppState { conn });
  HttpServer::new(move || App::new()
    .app_data(state.clone())
    .wrap(middleware::Compress::default())
    .service(show_fruits)
    .service(
      web::scope("/fruits")
        .service(show_fruits)
        .service(fruit_detail),
      ),
    )
    .bind(&server_url)?
    .run()
    .await?;

  Ok(())
}