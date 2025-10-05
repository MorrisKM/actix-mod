use actix_web::{web};
use crate::handlers::auth::{logout};
use crate::handlers::users::get_user;


pub fn get_auth_services() -> actix_web::Scope {
  return web::scope("/auth")
  .service(logout::logout)
  .service(get_user::fetch_user)
}
