use actix_web::{get, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::{models::{auth, errors, state}, utils::db::users};

#[get("/user/{name}/{id}/{email}")]
async fn fetch_user(pool: web::Data<SqlitePool>, id: web::Path<auth::PathParams>) -> Result<impl Responder, errors::HttpError> {
  // let state = app_data.state.lock().unwrap();
  // println!("the current state is {}", state);

  let user = auth::UserModel{
    id: String::from("some id"),
    firstname: "Morris".to_string(),
    lastname: "Munene".to_string(),
    password: "password".to_string(),
    email: "morris@email.com".to_string()
  };
  let user_json = serde_json::to_string_pretty(&user).unwrap();
  Ok(HttpResponse::Ok().body(user_json))
}