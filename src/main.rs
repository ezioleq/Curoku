use axum::Router;
use log::info;

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let app = Router::new()
        .merge(routes::client_routes())
        .merge(routes::api_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start listening");

    info!("Listening...");
    axum::serve(listener, app)
        .await
        .expect("Failed to start the server");
}
