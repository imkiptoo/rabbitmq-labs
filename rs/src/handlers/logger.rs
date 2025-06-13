use std::sync::Arc;
use warp::{Reply, Rejection};
use serde::{Deserialize, Serialize};
use crate::{AppState, WebSocketMessage};

#[derive(Deserialize)]
pub struct LogMessage {
    pub message: String,
}

#[derive(Serialize)]
pub struct LogResponse {
    pub success: bool,
    pub message: String,
}

pub async fn send_message(
    msg: LogMessage,
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    match state.rabbit.publish_message("message_logger", &msg.message).await {
        Ok(_) => {
            let ws_msg = WebSocketMessage {
                demo_type: "logger".to_string(),
                data: serde_json::json!({
                    "message": msg.message,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }),
            };
            
            let _ = state.broadcast_tx.send(ws_msg);
            
            Ok(warp::reply::json(&LogResponse {
                success: true,
                message: "Message sent successfully".to_string(),
            }))
        }
        Err(e) => {
            eprintln!("Failed to send message: {}", e);
            Ok(warp::reply::json(&LogResponse {
                success: false,
                message: format!("Failed to send message: {}", e),
            }))
        }
    }
}