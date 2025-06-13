use std::sync::Arc;
use warp::{Reply, Rejection};
use serde::{Deserialize, Serialize};
use crate::{AppState, WebSocketMessage};

#[derive(Deserialize)]
pub struct StatusRequest {}

#[derive(Serialize)]
pub struct StatusResponse {
    pub success: bool,
    pub timestamp: String,
    pub status: String,
    pub server_info: String,
}

pub async fn check_status(
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    let request_data = serde_json::json!({
        "type": "status_check",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    match state.rabbit.rpc_call(request_data.clone()).await {
        Ok(response) => {
            let ws_msg = WebSocketMessage {
                demo_type: "rpc".to_string(),
                data: serde_json::json!({
                    "type": "status_response",
                    "request": request_data,
                    "response": response
                }),
            };

            let _ = state.broadcast_tx.send(ws_msg);

            let status_messages = [
                "All systems operational",
                "Running smoothly",
                "Services healthy",
                "Everything looks good",
                "System status: OK",
            ];

            let random_status = status_messages[rand::random::<usize>() % status_messages.len()];

            Ok(warp::reply::json(&StatusResponse {
                success: true,
                timestamp: chrono::Utc::now().to_rfc3339(),
                status: random_status.to_string(),
                server_info: "RabbitMQ Demo Server v1.0".to_string(),
            }))
        }
        Err(e) => {
            let error_response = StatusResponse {
                success: false,
                timestamp: chrono::Utc::now().to_rfc3339(),
                status: format!("RPC Error: {}", e),
                server_info: "RabbitMQ Demo Server v1.0".to_string(),
            };

            let ws_msg = WebSocketMessage {
                demo_type: "rpc".to_string(),
                data: serde_json::json!({
                    "type": "status_error",
                    "error": e.to_string()
                }),
            };

            let _ = state.broadcast_tx.send(ws_msg);

            Ok(warp::reply::json(&error_response))
        }
    }
}