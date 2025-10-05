use actix_web::{web};

use crate::handlers::users::{add_user::add_user, delete_user, get_user};



pub fn get_user_services() -> actix_web::Scope {
  return web::scope("/user")
  .service(add_user)
  .service(delete_user::delete_user)
  .service(get_user::get_user)
}