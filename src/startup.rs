use crate::routes::*;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run(address: SocketAddr) -> Result<(), std::io::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "newsletter=info, newsletter=error".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let connection_string = std::env::var("DATABASE_URL").expect("Couldn't read database URL.");
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    axum::serve(
        TcpListener::bind(&address).await?,
        app(pool).into_make_service(),
    )
    .await
    .unwrap();

    Ok(())
}

pub fn app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/health-check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}
