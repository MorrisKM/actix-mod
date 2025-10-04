use std::sync::Mutex;

use actix_web::{web::{self, get}, App, HttpResponse, HttpServer, Responder};

mod api;
mod handlers;
mod models;
mod utils;

#[actix_web::main]
async fn main() {
    let data = web::Data::new(models::state::AppState{
        state: Mutex::new("init-state".to_string())
    });
    HttpServer::new(move || {
        App::new()
        .app_data(data.clone())
        .service(api::public::get_public_services())
        .service(api::auth::get_auth_services())
        .route("/hello", get().to(manual_hello))
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap()

}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}