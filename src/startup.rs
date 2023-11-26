use crate::routes::*;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

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
