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
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let body = "name=caio%20gomes&email=me%40caio.codes";
    let request = Request::builder()
        .method("POST")
        .uri("/subscriptions")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body.into())
        .unwrap();

    println!("request {:?}", request);
    let response = app().oneshot(request).await.unwrap();
    println!("response {:?}", response);

    assert!(response.status().is_success())
}
