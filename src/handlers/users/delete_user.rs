use actix_web::{delete, get, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::{models::{errors}, utils::db::users};

#[delete("/user/{id}")]
async fn delete_user(pool: web::Data<SqlitePool>, id: web::Path<String>) -> Result<impl Responder, errors::HttpError> {
  users::delete(id.to_string(), pool).await;
  Ok(HttpResponse::Ok().body("user deleted successfully"))
}