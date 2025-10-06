use actix_web::{guard, web};
use crate::handlers::auth::{logout};
use crate::handlers::users::get_user;
use crate::utils::guard::AuthorizationHeader;


pub fn get_auth_services() -> actix_web::Scope {
  return web::scope("/auth")
  .guard(AuthorizationHeader)
  .service(logout::logout)
  .service(get_user::get_user)
}
