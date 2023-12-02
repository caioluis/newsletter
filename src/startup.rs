use crate::routes::*;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{net::SocketAddr, time::Duration};

pub async fn run(address: SocketAddr) -> Result<(), std::io::Error> {
    let connection_string = std::env::var("DATABASE_URL").expect("Couldn't read database URL.");
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    axum::Server::bind(&address)
        .serve(app(pool).into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub fn app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/health-check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(pool)
}
