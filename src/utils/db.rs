use sqlx::{prelude::FromRow, Executor,SqlitePool};

pub async fn db() -> SqlitePool {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite://db.sqlite").await.unwrap();

    pool.execute("
        CREATE TABLE IF NOT EXISTS todos (
          id PRIMARY KEY AUTOINCREMENT,
          user_id INTEGER NULL,
          todo TEXT NOT NULL,
        )

        CREATE TABLE IF NOT EXISTS users (
          id TEXT PRIMARY KEY,
          firstname TEXT NOT NULL,
          lastname TEXT NOT NULL,
          password TEXT NOT NULL,
          email TEXT UNIQUE NOT NULL,
        )
    ").await.unwrap();
    pool
}


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
