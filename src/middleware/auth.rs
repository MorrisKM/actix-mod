use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};


pub struct TokenAuth;

impl<S, B> Transform<S, ServiceRequest> for TokenAuth
where 
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{}

pub struct TokenAuthMiddleware<S>{
  service: S,
}