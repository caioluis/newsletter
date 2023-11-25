use axum::body::Body;
use hyper::Request;
use newsletter::app;
use tower::ServiceExt;

#[tokio::test]
async fn health_check_works() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/health-check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response.status().is_success());
}
