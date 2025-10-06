use actix_web::{guard::Guard, http::{self, header}};

pub struct AuthorizationHeader;

impl Guard for AuthorizationHeader {
  fn check(&self, req: &actix_web::guard::GuardContext) -> bool {
      req.head()
      .headers()
      .contains_key(http::header::AUTHORIZATION)
  }
}