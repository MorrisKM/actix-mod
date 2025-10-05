use actix_web::{post, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::{models::{auth, errors}, utils::db::users};

#[post("/user/add")]
async fn add_user(user_req: web::Json<auth::UserModel>, pool: web::Data<SqlitePool>) -> impl Responder {
  let id = uuid::Uuid::new_v4().to_string();
  users::insert(user_req.0, pool, id).await;
  //   Ok(_) => Ok(HttpResponse::Ok().body("User added successfully")),
  //   Err(_) => Err(errors::HttpError::InternalError)
  // }
  HttpResponse::Ok().body("new user has been added successfully")
}