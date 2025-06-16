use std::sync::Arc;
use warp::{Reply, Rejection};
use serde::{Deserialize, Serialize};
use crate::{AppState, WebSocketMessage};
use tracing::{info, error, warn, debug, instrument};

#[derive(Deserialize)]
pub struct LogMessage {
    pub message: String,
}

#[derive(Serialize)]
pub struct LogResponse {
    pub success: bool,
    pub message: String,
}

#[instrument(skip(msg, state), fields(message_len = msg.message.len()))]
pub async fn send_message(
    msg: LogMessage,
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    info!("Received logger message request");
    debug!("Message content: {}", msg.message);
    
    match state.rabbit.publish_message("message_logger", &msg.message).await {
        Ok(_) => {
            info!("Message published to RabbitMQ successfully");
            
            let timestamp = chrono::Utc::now().to_rfc3339();
            let ws_msg = WebSocketMessage {
                demo_type: "logger".to_string(),
                data: serde_json::json!({
                    "message": msg.message,
                    "timestamp": timestamp
                }),
            };
            
            if let Err(_) = state.broadcast_tx.send(ws_msg) {
                warn!("No WebSocket clients connected for logger message");
            } else {
                debug!("Logger message broadcasted to WebSocket clients");
            }
            
            info!("Logger message processed successfully");
            Ok(warp::reply::json(&LogResponse {
                success: true,
                message: "Message sent successfully".to_string(),
            }))
        }
        Err(e) => {
            error!("Failed to send message to RabbitMQ: {}", e);
            Ok(warp::reply::json(&LogResponse {
                success: false,
                message: format!("Failed to send message: {}", e),
            }))
        }
    }
}