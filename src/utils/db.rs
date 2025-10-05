use sqlx::{prelude::FromRow, Executor,SqlitePool};




pub mod users {
  use actix_web::web;
use sqlx::SqlitePool;

use crate::models::auth::UserModel;
  pub async fn insert(user: UserModel, pool: web::Data<SqlitePool>, id: String) {
    sqlx::query("INSERT INTO users (firstname, lastname, password, email, id)")
    .bind(user.firstname)
    .bind(user.lastname)
    .bind(user.password)
    .bind(user.email)
    .bind(id)
    .execute(pool.get_ref()).await.unwrap();
  }

  pub async fn get(id: String, pool: web::Data<SqlitePool>) -> UserModel {
    sqlx::query_as("SELECT firstname, lastname, email, id FROM users WHERE id = ?1").bind(id).fetch_one(pool.get_ref()).await.unwrap()
  }
  pub async fn delete(id: String, pool: web::Data<SqlitePool>) {
    sqlx::query("DELETE FROM users WHERE id = ?1").bind(id).execute(pool.get_ref()).await.unwrap();
  }
  pub async fn getall(pool: web::Data<SqlitePool>) -> Vec<UserModel> {
    sqlx::query_as("SELECT firstname, lastname, email, id FROM users WHERE id = ?1").fetch_all(pool.get_ref()).await.unwrap()
  }
  pub async fn update(user: UserModel, pool: web::Data<SqlitePool>) {
    sqlx::query("INSERT INTO users (id, firstname, lastname, password, email) WHERE id = ?1")
    .bind(user.id)
    .bind(user.firstname)
    .bind(user.lastname)
    .bind(user.password)
    .bind(user.email)
    .execute(pool.get_ref()).await.unwrap();
  }
}
