use std::sync::Arc;

use axum::extract::Form;
use axum::extract::State;
use axum::http::StatusCode;
use chrono::Utc;
use uuid::Uuid;

use crate::startup::AppState;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[axum::debug_handler]
pub async fn subscribe(
    State(state): State<Arc<AppState>>,
    Form(form): Form<FormData>,
) -> StatusCode {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&state.pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
