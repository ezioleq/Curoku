use axum::{
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

use crate::handlers;

pub fn client_routes() -> Router {
    Router::new().nest_service(
        "/",
        get_service(ServeDir::new("./client/dist")).handle_error(|error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal server error: {}", error),
            )
        }),
    )
}

pub fn api_routes() -> Router {
    Router::new().route("/api/connect", get(handlers::handle_ws_upgrade))
}
