use actix_web::{post, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::{models::{auth::{self, UserModel}, state}, utils::{db::users::get_by_email, jwt::jwt::encode}};

#[post("/login")]
async fn login(app_data: web::Data<state::AppState>, pool: web::Data<SqlitePool>,req: web::Json<auth::LoginReq>) -> impl Responder {
  let mut state = app_data.state.lock().unwrap();
  *state = "login".to_string();
  
  let user = get_by_email(req.email.to_string(), pool).await;
  let token = encode(user);
  HttpResponse::Ok().body(token)
}



