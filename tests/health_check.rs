use axum::body::Body;
use hyper::Request;
use newsletter::startup::app;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower::ServiceExt;

#[tokio::test]
async fn health_check_works() {
    let connection_string = std::env::var("DATABASE_URL").expect("Couldn't read database URL.");
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let response = app(pool)
        .oneshot(
            Request::builder()
                .uri("/health-check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

// #[sqlx::test]
// async fn it_subscribes_new_member() {
//     let configuration = get_configuration().expect("Failed to read configuration.");
//     let pool = PgPoolOptions::new()
//         .acquire_timeout(Duration::from_secs(3))
//         .connect(&configuration.database.connection_string())
//         .await
//         .expect("Failed to connect to Postgres.");
//     let body = "name=caio%20gomes&email=me%40caio.codes";
//     let request = Request::builder()
//         .method("POST")
//         .uri("/subscriptions")
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body.into())
//         .unwrap();
//     let response = app(pool).oneshot(request).await.unwrap();

//     assert!(response.status().is_success())
// }
