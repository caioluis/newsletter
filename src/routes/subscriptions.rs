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
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
