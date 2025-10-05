use std::sync::Mutex;

use actix_web::{web::{self, get}, App, HttpResponse, HttpServer, Responder};
use tracing::{info, subscriber, Level};
use tracing_subscriber::FmtSubscriber;
use dotenv::dotenv;
use std::env;
use sqlx::{prelude::FromRow, Executor,SqlitePool};

mod api;
mod handlers;
mod models;
mod utils;

#[actix_web::main]
async fn main() {
    dotenv().ok();
    let data = web::Data::new(models::state::AppState{
        state: Mutex::new("init-state".to_string())
    });

    // let conn = env::var("DATABASE_CONNECTION_STRING").expect("the database connection string was not set");
    // println!("connection string: {conn}");

    let pool = db().await;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("error setting global subscriber for tracing");
    info!("Starting server at port: 3000");
    HttpServer::new(move || {
        App::new()
        .app_data(data.clone())
        .app_data(web::Data::new(pool.clone()))
        .service(api::public::get_public_services())
        .service(api::users::get_user_services())
        .service(api::auth::get_auth_services())
        .route("/hello", get().to(manual_hello))
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    //.worker() this will specify the number of cores to use in the app
    .run()
    .await
    .unwrap()

}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

async fn db() -> SqlitePool {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite://db.sqlite").await.unwrap();

    pool.execute("
        CREATE TABLE IF NOT EXISTS todos (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          user_id INTEGER NULL,
          todo TEXT NOT NULL
        );
    ").await.unwrap();

    pool.execute("
        CREATE TABLE IF NOT EXISTS users (
          id TEXT PRIMARY KEY,
          firstname TEXT NOT NULL,
          lastname TEXT NOT NULL,
          password TEXT NOT NULL,
          email TEXT UNIQUE NOT NULL
        )
    ").await.unwrap();
    pool
}