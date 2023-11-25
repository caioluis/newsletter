use axum::{
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct UserInfo {
    email: String,
    name: String,
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(Form(_user_info): Form<UserInfo>) -> StatusCode {
    StatusCode::OK
}

pub async fn run() -> Result<(), std::io::Error> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub fn app() -> Router {
    Router::new()
        .route("/health-check", get(health_check))
        .route("/subscriptions", post(subscribe))
}
