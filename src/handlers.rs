use axum::{extract::WebSocketUpgrade, response::Response};
use log::info;

pub async fn handle_ws_upgrade(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|mut socket| async move {
        while let Some(msg) = socket.recv().await {
            let msg = if let Ok(msg) = msg {
                info!("Received message: {}", msg.to_text().unwrap());
                msg
            } else {
                info!("Client disconnected");
                return;
            };

            if socket.send(msg).await.is_err() {
                info!("Failed to send message to a client");
                return;
            }
        }
    })
}
