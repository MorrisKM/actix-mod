use actix_web::{get, web, HttpResponse, Responder};

use crate::models::{state, auth};

#[get("/user/{name}/{id}/{email}")]
async fn fetch_user(app_data: web::Data<state::AppState>, path: web::Path<auth::PathParams>) -> impl Responder {
  let state = app_data.state.lock().unwrap();
  println!("the current state is {}", state);
  HttpResponse::Ok().body(format!("Hello {}, your id is {} and email {}", path.name, path.id, path.email))
}