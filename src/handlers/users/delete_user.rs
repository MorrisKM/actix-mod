use actix_web::{get, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::{models::{errors}, utils::db::users};

#[get("/user/{name}/{id}/{email}")]
async fn fetch_user(pool: web::Data<SqlitePool>, id: web::Path<String>) -> Result<impl Responder, errors::HttpError> {
  users::delete(id.to_string(), pool).await;
  Ok(HttpResponse::Ok().body("user deleted successfully"))
}