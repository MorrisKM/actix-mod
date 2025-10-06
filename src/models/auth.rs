use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginReq {
  pub password: String,
  pub email: String
}

#[derive(Deserialize, Debug, Serialize, sqlx::FromRow)]
pub struct UserModel {
  pub id: String,
  pub firstname: String,
  pub lastname: String,
  pub password: String,
  pub email: String,
}


#[derive(Deserialize)]
pub struct PathParams{
  pub name: String,
  pub id: String,
  pub email: String
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Claims {
  pub custom_claim: UserModel,
  pub iss: String,
  pub sub: String,
  pub aud: String,
  pub exp: u64
}