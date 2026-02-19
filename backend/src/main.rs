use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(greet_user));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn greet_user() -> String {
    // print to console
    println!("Hello Yusef, only you can see this");
    String::from("Hello Yusef, everyone accessing this site sees this  !")
}
