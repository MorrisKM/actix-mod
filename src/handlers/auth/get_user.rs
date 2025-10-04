use actix_web::{get, web, HttpResponse, Responder};

use crate::models::{state, auth, errors};

#[get("/user/{name}/{id}/{email}")]
async fn fetch_user(app_data: web::Data<state::AppState>, path: web::Path<auth::PathParams>) -> Result<impl Responder, errors::HttpError> {
  // let state = app_data.state.lock().unwrap();
  // println!("the current state is {}", state);
  let user = auth::UserModel{
    firstname: "Morris".to_string(),
    lastname: "Munene".to_string(),
    password: "password".to_string(),
    email: "morris@email.com".to_string()
  };
  let user_json = serde_json::to_string_pretty(&user).unwrap();
  Ok(HttpResponse::Ok().body(user_json))
}