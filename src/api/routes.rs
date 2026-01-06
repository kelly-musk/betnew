use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::api::handlers;

pub fn create_router() -> Router {
    Router::new()
        // Authentication
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/refresh", post(handlers::auth::refresh_token))
        // Predictions
        .route(
            "/api/predictions",
            post(handlers::predictions::create_prediction)
                .get(handlers::predictions::get_user_predictions),
        )
        .route(
            "/api/predictions/:id",
            get(handlers::predictions::get_prediction)
                .put(handlers::predictions::update_prediction),
        )
        // Events
        .route(
            "/api/events",
            get(handlers::events::list_events).post(handlers::events::create_event),
        )
        .route(
            "/api/events/:id",
            get(handlers::events::get_event).put(handlers::events::update_event),
        )
        // Leaderboard
        .route(
            "/api/leaderboard",
            get(handlers::leaderboard::get_leaderboard),
        )
        // WebSocket for real-time updates
        .route("/ws", get(handlers::websocket::handler))
}
