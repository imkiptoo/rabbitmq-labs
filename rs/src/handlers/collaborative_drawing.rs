use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::reply::Json;

use crate::{AppState, WebSocketMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawingEvent {
    pub event_type: String,
    pub user_id: String,
    pub username: String,
    pub x: f64,
    pub y: f64,
    pub prev_x: Option<f64>,
    pub prev_y: Option<f64>,
    pub color: String,
    pub brush_size: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorEvent {
    pub user_id: String,
    pub username: String,
    pub x: f64,
    pub y: f64,
    pub color: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct DrawingRequest {
    pub user_id: String,
    pub username: String,
    pub x: f64,
    pub y: f64,
    pub prev_x: Option<f64>,
    pub prev_y: Option<f64>,
    pub color: String,
    pub brush_size: f64,
    pub event_type: String,
}

#[derive(Debug, Deserialize)]
pub struct CursorMoveRequest {
    pub user_id: String,
    pub username: String,
    pub x: f64,
    pub y: f64,
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteStrokesRequest {
    pub user_id: String,
    pub username: String,
}

pub async fn handle_drawing_event(
    request: DrawingRequest,
    state: Arc<AppState>,
) -> Result<Json, warp::Rejection> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    
    let drawing_event = DrawingEvent {
        event_type: request.event_type.clone(),
        user_id: request.user_id.clone(),
        username: request.username.clone(),
        x: request.x,
        y: request.y,
        prev_x: request.prev_x,
        prev_y: request.prev_y,
        color: request.color.clone(),
        brush_size: request.brush_size,
        timestamp: timestamp.clone(),
    };

    // Save to Redis for persistence
    let event_json = serde_json::to_string(&drawing_event).unwrap_or_default();
    if let Err(e) = state.redis.append_drawing_event(&event_json).await {
        tracing::error!("Failed to save drawing event to Redis: {}", e);
    }

    // Publish to RabbitMQ fanout exchange for real-time collaboration
    if let Err(e) = state.rabbit.publish_fanout("collaborative_drawing", &event_json).await {
        tracing::error!("Failed to publish drawing event to RabbitMQ: {}", e);
    }

    // Broadcast via WebSocket to all connected clients
    let websocket_message = WebSocketMessage {
        demo_type: "collaborative_drawing".to_string(),
        data: serde_json::json!({
            "action": "drawing_event",
            "event": drawing_event,
            "timestamp": timestamp
        }),
    };

    if let Err(_) = state.broadcast_tx.send(websocket_message) {
        tracing::warn!("No WebSocket clients connected for drawing event");
    }

    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "message": "Drawing event processed successfully"
    })))
}

pub async fn handle_cursor_move(
    request: CursorMoveRequest,
    state: Arc<AppState>,
) -> Result<Json, warp::Rejection> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    
    let cursor_event = CursorEvent {
        user_id: request.user_id.clone(),
        username: request.username.clone(),
        x: request.x,
        y: request.y,
        color: request.color.clone(),
        timestamp: timestamp.clone(),
    };

    // Broadcast cursor position via WebSocket only (no need for persistence)
    let websocket_message = WebSocketMessage {
        demo_type: "collaborative_drawing".to_string(),
        data: serde_json::json!({
            "action": "cursor_move",
            "cursor": cursor_event,
            "timestamp": timestamp
        }),
    };

    if let Err(_) = state.broadcast_tx.send(websocket_message) {
        tracing::warn!("No WebSocket clients connected for cursor move");
    }

    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "message": "Cursor position updated successfully"
    })))
}

pub async fn clear_canvas(state: Arc<AppState>) -> Result<Json, warp::Rejection> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    
    // Clear Redis storage
    if let Err(e) = state.redis.clear_canvas().await {
        tracing::error!("Failed to clear canvas in Redis: {}", e);
    }
    
    // Publish clear event to RabbitMQ
    let clear_event = serde_json::json!({
        "action": "clear_canvas",
        "timestamp": timestamp
    });
    
    if let Err(e) = state.rabbit.publish_fanout("collaborative_drawing", &clear_event.to_string()).await {
        tracing::error!("Failed to publish clear canvas to RabbitMQ: {}", e);
    }

    // Broadcast via WebSocket
    let websocket_message = WebSocketMessage {
        demo_type: "collaborative_drawing".to_string(),
        data: clear_event,
    };

    if let Err(_) = state.broadcast_tx.send(websocket_message) {
        tracing::warn!("No WebSocket clients connected for canvas clear");
    }

    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "message": "Canvas cleared successfully"
    })))
}

pub async fn load_canvas_state(state: Arc<AppState>) -> Result<Json, warp::Rejection> {
    match state.redis.get_drawing_state().await {
        Ok(Some(drawing_events)) => {
            // Parse the JSON string to ensure it's valid
            match serde_json::from_str::<serde_json::Value>(&drawing_events) {
                Ok(events) => Ok(warp::reply::json(&serde_json::json!({
                    "success": true,
                    "events": events
                }))),
                Err(_) => Ok(warp::reply::json(&serde_json::json!({
                    "success": true,
                    "events": []
                })))
            }
        },
        Ok(None) => Ok(warp::reply::json(&serde_json::json!({
            "success": true,
            "events": []
        }))),
        Err(e) => {
            tracing::error!("Failed to load canvas state from Redis: {}", e);
            Ok(warp::reply::json(&serde_json::json!({
                "success": false,
                "error": "Failed to load canvas state",
                "events": []
            })))
        }
    }
}

pub async fn delete_user_strokes(
    request: DeleteStrokesRequest,
    state: Arc<AppState>,
) -> Result<Json, warp::Rejection> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    
    // Remove user's strokes from Redis
    if let Err(e) = state.redis.delete_user_strokes(&request.user_id).await {
        tracing::error!("Failed to delete user strokes from Redis: {}", e);
        return Ok(warp::reply::json(&serde_json::json!({
            "success": false,
            "error": "Failed to delete strokes"
        })));
    }
    
    // Broadcast delete event to all clients
    let delete_event = serde_json::json!({
        "action": "delete_user_strokes",
        "user_id": request.user_id,
        "username": request.username,
        "timestamp": timestamp
    });
    
    if let Err(e) = state.rabbit.publish_fanout("collaborative_drawing", &delete_event.to_string()).await {
        tracing::error!("Failed to publish delete strokes to RabbitMQ: {}", e);
    }

    let websocket_message = WebSocketMessage {
        demo_type: "collaborative_drawing".to_string(),
        data: delete_event,
    };

    if let Err(_) = state.broadcast_tx.send(websocket_message) {
        tracing::warn!("No WebSocket clients connected for delete strokes");
    }

    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "message": "User strokes deleted successfully"
    })))
}