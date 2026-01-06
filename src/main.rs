use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber;
use tokio::sync::broadcast;

mod api;
mod core;
mod infrastructure;
mod config;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load configuration
    let config = config::Settings::new().expect("Failed to load configuration");
    
    // Setup database connection pool
    let db_pool = infrastructure::database::connect(&config.database.url)
        .await
        .expect("Failed to connect to database");
    
    // Setup Redis
    let redis_client = infrastructure::cache::RedisClient::new(&config.redis.url)
        .await
        .expect("Failed to connect to Redis");
    
    // Create broadcast channel for WebSocket updates
    let (tx, _) = broadcast::channel(100);
    let ws_state = api::handlers::websocket::WebSocketState { tx };
    
    // Build application state
    let app_state = api::state::AppState {
        db_pool,
        redis_client,
        ws_state: std::sync::Arc::new(ws_state),
        config,
    };
    
    // Create router
    let app = api::routes::create_router().with_state(app_state);
    
    // Run migrations
    infrastructure::database::run_migrations(&db_pool)
        .await
        .expect("Failed to run migrations");
    
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
