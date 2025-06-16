use std::sync::Arc;
use warp::{Reply, Rejection};
use serde::{Deserialize, Serialize};
use crate::{AppState, WebSocketMessage};
use tracing::{info, error, warn, debug, instrument};

#[derive(Deserialize)]
pub struct StatusRequest {}

#[derive(Serialize)]
pub struct StatusResponse {
    pub success: bool,
    pub timestamp: String,
    pub status: String,
    pub server_info: String,
}

#[instrument(skip(state))]
pub async fn check_status(
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    info!("Received RPC status check request");
    
    let timestamp = chrono::Utc::now().to_rfc3339();
    let request_data = serde_json::json!({
        "type": "status_check",
        "timestamp": timestamp
    });
    
    debug!("Making RPC call for status check");
    match state.rabbit.rpc_call(request_data.clone()).await {
        Ok(response) => {
            info!("RPC status check completed successfully");
            
            let ws_msg = WebSocketMessage {
                demo_type: "rpc".to_string(),
                data: serde_json::json!({
                    "type": "status_response",
                    "request": request_data,
                    "response": response
                }),
            };

            if let Err(_) = state.broadcast_tx.send(ws_msg) {
                warn!("No WebSocket clients for RPC status response");
            } else {
                debug!("RPC status response broadcasted to WebSocket clients");
            }

            let status_messages = [
                "All systems operational",
                "Running smoothly",
                "Services healthy",
                "Everything looks good",
                "System status: OK",
            ];

            let random_status = status_messages[rand::random::<usize>() % status_messages.len()];
            info!("Returning status: {}", random_status);

            Ok(warp::reply::json(&StatusResponse {
                success: true,
                timestamp: chrono::Utc::now().to_rfc3339(),
                status: random_status.to_string(),
                server_info: "RabbitMQ Demo Server v1.0".to_string(),
            }))
        }
        Err(e) => {
            error!("RPC status check failed: {}", e);
            
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

            if let Err(_) = state.broadcast_tx.send(ws_msg) {
                warn!("No WebSocket clients for RPC error notification");
            } else {
                debug!("RPC error notification broadcasted to WebSocket clients");
            }

            Ok(warp::reply::json(&error_response))
        }
    }
}