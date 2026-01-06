use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::core::models::{CreatePredictionRequest, Prediction};
use crate::infrastructure::database::DbPool;

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreatePredictionRequest {
    #[validate(length(min = 1, max = 200))]
    pub event_id: Uuid,

    #[validate(length(min = 1, max = 100))]
    pub predicted_outcome: String,

    #[validate(range(min = 0, max = 100))]
    pub confidence: i32,
}

pub async fn create_prediction(
    State(pool): State<DbPool>,
    user_id: crate::api::middleware::auth::AuthenticatedUser,
    Json(payload): Json<CreatePredictionRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    payload.validate()?;

    // Check if event is still open
    let event = sqlx::query_as!(
        Event,
        "SELECT * FROM events WHERE id = $1 AND status = 'open' AND closing_time > NOW()",
        payload.event_id
    )
    .fetch_optional(&pool)
    .await?;

    if event.is_none() {
        return Err(AppError::EventClosed);
    }

    // Create prediction
    let prediction_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO predictions 
        (id, user_id, event_id, predicted_outcome, confidence, created_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        "#,
        prediction_id,
        user_id.0,
        payload.event_id,
        payload.predicted_outcome,
        payload.confidence
    )
    .execute(&pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": prediction_id,
            "message": "Prediction created successfully"
        })),
    ))
}
