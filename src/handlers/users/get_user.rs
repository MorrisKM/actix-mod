use actix_web::{get, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::{models::{auth, errors, state}, utils::db::users};

#[get("/user/{id}")]
async fn get_user(pool: web::Data<SqlitePool>, id: web::Path<String>) -> impl Responder {
  let user = users::get(id.to_string(), pool).await;
  let user_json = serde_json::to_string_pretty(&user).unwrap();
  HttpResponse::Ok().body(user_json)
}