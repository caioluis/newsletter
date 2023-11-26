use axum::{http::StatusCode, Form};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct UserInfo {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(Form(_user_info): Form<UserInfo>) -> StatusCode {
    StatusCode::OK
}
