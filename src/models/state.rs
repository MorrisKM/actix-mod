use std::sync::Mutex;

use sqlx::SqlitePool;

pub struct AppState {
  pub state: Mutex<String>,
  // pub pool: SqlitePool
}