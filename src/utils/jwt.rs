pub mod jwt {
    use std::{env, time::{Duration, SystemTime}};

    use crate::models::auth::{Claims, UserModel};

  pub fn encode(user: UserModel) -> String {
    //create an expiration time
    let exp_time = std::time::SystemTime::now()
    .checked_add(Duration::from_secs(360))
    .unwrap()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs();

    let claims = Claims{
      //user
      custom_claim: user,
      //issuer
      iss: "https/github/MorrisKM".to_string(),
      //subject
      sub: "rust_track".to_string(),
      //audience
      aud: "rust_track".to_string(),
      //expiration time
      exp: exp_time
    };

    //create a header
    let header = jsonwebtoken::Header::default();
    let secret = env::var("JWT_SECRETS").expect("JWT SECRET was not set");
    let key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());

    let token = jsonwebtoken::encode(&header, &claims, &key).unwrap();
    token
  }
  
  pub fn decode(token: &str) -> bool{
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);

    let secret = env::var("JWT_SECRETS").expect("JWT SECRET was not set");
    validation.set_audience(&vec!["rust_track".to_string()]);
    let key = jsonwebtoken::DecodingKey::from_secret(secret.as_bytes());

    match jsonwebtoken::decode::<Claims>(token, &key, &validation){
      Ok(token) => {
        return token.claims.iss == "https/github/MorrisKM".to_string();
      },
      Err(_) => return false
    }

  }
}