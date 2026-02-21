use std::fmt::format;

use axum::{Json, extract::State, http::StatusCode};
use serde_json::{Value, json};
use sqlx::{self, Error, Row, SqlitePool, pool};
pub async fn init_db() -> Result<SqlitePool, Error> {
    let database_url = std::env::var("DATABASE_URL").expect("Database url must be set");
    let connection_pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&connection_pool).await?;
    Ok(connection_pool)
}
pub async fn get_user(State(pool): State<SqlitePool>) -> Result<Json<Value>, StatusCode> {
    let rows = sqlx::query("select * from yuseftest")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut json_out: String = String::new();
    for row in &rows {
        let id: i64 = row.get("id");
        let name: String = row.get("name");
        println!("{}-{}", id, name);
        json_out = String::from(format!("{id}-{name}"))
    }
    Ok(Json(json!({ "rows": rows.len(),"user:": json_out })))
}
