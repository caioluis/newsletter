use axum::{extract::State, http::StatusCode, Form};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct UserInfo {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(State(pool): State<PgPool>, Form(user_info): Form<UserInfo>) -> StatusCode {
    let request_id = Uuid::new_v4();
    tracing::info!(
        "request_id {} - Adding '{}' '{}'as new subscriber.",
        request_id,
        user_info.email,
        user_info.name
    );
    tracing::info!(
        "request_id {} - Saving new subscriver details in the database",
        request_id
    );
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        user_info.email,
        user_info.name,
        Utc::now()
    )
    .execute(&pool)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
