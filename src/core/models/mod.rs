pub struct user{
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub total_points: i32,
    pub current_streak: i32,
    pub avatar_url: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub options: serde_json::Value, // JSON array of options
    pub closing_time: chrono::DateTime<chrono::Utc>,
    pub status: EventStatus, // Enum: Open, Closed, Resolved
    pub actual_outcome: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "event_status", rename_all = "lowercase")]
pub enum EventStatus {
    Open,
    Closed,
    Resolved,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Prediction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Uuid,
    pub predicted_outcome: String,
    pub confidence: i32, // 0-100
    pub points_awarded: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub is_correct: Option<bool>,
}