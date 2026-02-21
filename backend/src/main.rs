mod db;
use crate::db::{get_user, init_db};
use axum::{Json, Router, routing::get};
use sqlx::{Error, SqlitePool, error::BoxDynError};
use tokio::net::TcpListener;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let connection_pool = init_db().await?;
    let app = Router::new()
        .route("/", get(greet_user))
        .route("/user", get(get_user))
        .route("/json", get(testing_json))
        .with_state(connection_pool);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}
async fn testing_json() -> Json<String> {
    Json("Muie".to_string())
}

async fn greet_user() -> String {
    // print to console
    println!("Hello Yusef, only you can see this");
    String::from("Hello Yusef, everyone accessing this site sees this  !")
}
