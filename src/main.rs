use std::sync::Mutex;

use actix_web::{web::{self, get}, App, HttpResponse, HttpServer, Responder};
use tracing::{info, subscriber, Level};
use tracing_subscriber::FmtSubscriber;
use dotenv::dotenv;

use crate::{handlers::chat::ws, middleware::auth::TokenAuth, utils::db::db};

mod api;
mod handlers;
mod models;
mod utils;
mod middleware;

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

    // let user = UserModel {
    //     firstname: "Morris".to_string(),
    //     lastname: "Munene".to_string(),
    //     email: "morris@email.com".to_string(),
    //     password: "password".to_string(),
    //     id: "uuid".to_string()
    // };
    // let token = utils::jwt::jwt::encode(user);
    // println!("Token gen: {}", &token);
    // let res = utils::jwt::jwt::decode(&token);
    // println!("validation: {}", res);

    info!("Starting server at port: 3000");
    HttpServer::new(move || {
        App::new()
        .app_data(data.clone())
        .app_data(web::Data::new(pool.clone()))
        .service(api::public::get_public_services())
        //.wrap(TokenAuth)
        .service(api::users::get_user_services())
        .service(api::auth::get_auth_services())
        .route("/hello", get().to(manual_hello))
        .route("/ws", web::get().to(ws))
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    //.worker() this will specify the number of cores to use in the app
    .run()
    .await
    .unwrap()

}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

