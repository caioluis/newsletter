use crate::routes::*;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

pub async fn run(address: SocketAddr) -> Result<(), std::io::Error> {
    axum::Server::bind(&address)
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
