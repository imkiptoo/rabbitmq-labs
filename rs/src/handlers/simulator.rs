use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::reply::Json;

use crate::{AppState, WebSocketMessage};
use tracing::{info, warn, debug, instrument};

#[derive(Debug, Deserialize)]
pub struct SimulateRequest {
    pub demo_type: String,
    pub message_data: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct SimulateResponse {
    pub success: bool,
    pub message: String,
    pub flow_id: String,
}

#[derive(Debug, Serialize)]
pub struct QueueStatsResponse {
    pub queues: Vec<QueueInfo>,
    pub exchanges: Vec<ExchangeInfo>,
}

#[derive(Debug, Serialize)]
pub struct QueueInfo {
    pub name: String,
    pub message_count: u32,
    pub consumer_count: u32,
    pub queue_type: String,
}

#[derive(Debug, Serialize)]
pub struct ExchangeInfo {
    pub name: String,
    pub exchange_type: String,
    pub durable: bool,
}

#[instrument(skip(request, state), fields(demo_type = %request.demo_type, flow_id))]
pub async fn simulate_message(
    request: SimulateRequest,
    state: Arc<AppState>,
) -> Result<Json, warp::Rejection> {
    let flow_id = uuid::Uuid::new_v4().to_string();
    tracing::Span::current().record("flow_id", &flow_id);
    
    info!("Starting message flow simulation for demo type: {}", request.demo_type);
    
    let websocket_message = WebSocketMessage {
        demo_type: "simulator".to_string(),
        data: serde_json::json!({
            "action": "message_flow",
            "demo_type": request.demo_type,
            "flow_id": flow_id,
            "message_data": request.message_data,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }),
    };

    let response = match request.demo_type.as_str() {
        "logger" => {
            debug!("Simulating logger flow");
            simulate_logger_flow(&state, &request.message_data, &flow_id).await
        }
        "workers" => {
            debug!("Simulating workers flow");
            simulate_workers_flow(&state, &request.message_data, &flow_id).await
        }
        "game" => {
            debug!("Simulating game flow");
            simulate_game_flow(&state, &request.message_data, &flow_id).await
        }
        "rpc" => {
            debug!("Simulating RPC flow");
            simulate_rpc_flow(&state, &request.message_data, &flow_id).await
        }
        _ => {
            warn!("Unknown demo type requested: {}", request.demo_type);
            return Ok(warp::reply::json(&SimulateResponse {
                success: false,
                message: "Unknown demo type".to_string(),
                flow_id,
            }));
        }
    };

    if let Err(_) = state.broadcast_tx.send(websocket_message) {
        warn!("No WebSocket clients connected for simulation start message");
    } else {
        debug!("Simulation start message broadcasted to WebSocket clients");
    }
    
    info!("Message flow simulation completed for demo type: {}", request.demo_type);
    Ok(warp::reply::json(&response))
}

pub async fn get_queue_stats(_state: Arc<AppState>) -> Result<Json, warp::Rejection> {
    let queues = vec![
        QueueInfo {
            name: "messages".to_string(),
            message_count: 0,
            consumer_count: 1,
            queue_type: "classic".to_string(),
        },
        QueueInfo {
            name: "work_queue".to_string(),
            message_count: 0,
            consumer_count: 3,
            queue_type: "classic".to_string(),
        },
        QueueInfo {
            name: "rpc_queue".to_string(),
            message_count: 0,
            consumer_count: 1,
            queue_type: "classic".to_string(),
        },
    ];

    let exchanges = vec![
        ExchangeInfo {
            name: "game_fanout".to_string(),
            exchange_type: "fanout".to_string(),
            durable: true,
        },
        ExchangeInfo {
            name: "amq.direct".to_string(),
            exchange_type: "direct".to_string(),
            durable: true,
        },
    ];

    let response = QueueStatsResponse { queues, exchanges };
    Ok(warp::reply::json(&response))
}

async fn simulate_logger_flow(
    state: &Arc<AppState>,
    message_data: &serde_json::Value,
    flow_id: &str,
) -> SimulateResponse {
    let message = message_data.get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("Simulated message");

    let steps = vec![
        ("producer", "Publishing message to queue"),
        ("queue", "Message stored in 'messages' queue"),
        ("consumer", "Message consumed and processed"),
    ];

    send_flow_steps(state, "logger", flow_id, &steps).await;

    SimulateResponse {
        success: true,
        message: format!("Simulated logger flow for: {}", message),
        flow_id: flow_id.to_string(),
    }
}

async fn simulate_workers_flow(
    state: &Arc<AppState>,
    message_data: &serde_json::Value,
    flow_id: &str,
) -> SimulateResponse {
    let number = message_data.get("number")
        .and_then(|v| v.as_u64())
        .unwrap_or(42);

    let worker_id = (number % 3) + 1;
    let worker_node = format!("worker{}", worker_id);
    let worker_desc = format!("Worker {} processing number {}", worker_id, number);
    let steps = vec![
        ("producer", "Publishing work to queue"),
        ("queue", "Work stored in 'work_queue'"),
        (worker_node.as_str(), worker_desc.as_str()),
    ];

    send_flow_steps(state, "workers", flow_id, &steps).await;

    SimulateResponse {
        success: true,
        message: format!("Simulated work queue flow for number: {}", number),
        flow_id: flow_id.to_string(),
    }
}

async fn simulate_game_flow(
    state: &Arc<AppState>,
    message_data: &serde_json::Value,
    flow_id: &str,
) -> SimulateResponse {
    let player = message_data.get("player")
        .and_then(|v| v.as_str())
        .unwrap_or("Player");

    let steps = vec![
        ("producer", "Publishing score update"),
        ("exchange", "Fanout exchange broadcasting"),
        ("queue1", "Message to player queue 1"),
        ("queue2", "Message to player queue 2"),
        ("queue3", "Message to player queue 3"),
        ("player1", "Player 1 receives update"),
        ("player2", "Player 2 receives update"),
        ("player3", "Player 3 receives update"),
    ];

    send_flow_steps(state, "game", flow_id, &steps).await;

    SimulateResponse {
        success: true,
        message: format!("Simulated fanout flow for player: {}", player),
        flow_id: flow_id.to_string(),
    }
}

async fn simulate_rpc_flow(
    state: &Arc<AppState>,
    _message_data: &serde_json::Value,
    flow_id: &str,
) -> SimulateResponse {
    let steps = vec![
        ("client", "Sending RPC request"),
        ("request_queue", "Request in 'rpc_queue'"),
        ("server", "Server processing request"),
        ("reply_queue", "Reply in temporary queue"),
        ("client", "Client receives response"),
    ];

    send_flow_steps(state, "rpc", flow_id, &steps).await;

    SimulateResponse {
        success: true,
        message: "Simulated RPC request/reply flow".to_string(),
        flow_id: flow_id.to_string(),
    }
}

async fn send_flow_steps(
    state: &Arc<AppState>,
    demo_type: &str,
    flow_id: &str,
    steps: &[(&str, &str)],
) {
    for (i, (node, description)) in steps.iter().enumerate() {
        let step_message = WebSocketMessage {
            demo_type: "simulator".to_string(),
            data: serde_json::json!({
                "action": "flow_step",
                "demo_type": demo_type,
                "flow_id": flow_id,
                "step": i,
                "node": node,
                "description": description,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
        };

        if let Err(_) = state.broadcast_tx.send(step_message) {
            tracing::warn!("No WebSocket clients connected for flow step");
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
}