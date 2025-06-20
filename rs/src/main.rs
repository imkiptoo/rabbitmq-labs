use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use warp::{Filter, ws::WebSocket};
use serde::{Deserialize, Serialize};
use tracing::{info, error, warn, debug};

mod handlers;
mod rabbitmq;
mod redis_store;

use handlers::*;
use rabbitmq::RabbitMQConnection;
use redis_store::RedisStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub demo_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub rabbit: Arc<RabbitMQConnection>,
    pub broadcast_tx: broadcast::Sender<WebSocketMessage>,
    pub game_scores: Arc<Mutex<HashMap<String, u32>>>,
    pub redis: Arc<RedisStore>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting RabbitMQ Demo Server");

    info!("Initializing RabbitMQ connection");
    let rabbit = Arc::new(RabbitMQConnection::new().await.expect("Failed to connect to RabbitMQ"));
    info!("RabbitMQ connection established successfully");

    info!("Initializing Redis connection");
    let redis = Arc::new(RedisStore::new().await.expect("Failed to connect to Redis"));
    info!("Redis connection established successfully");

    info!("Setting up broadcast channel and game state");
    let (broadcast_tx, _) = broadcast::channel(100);
    let game_scores = Arc::new(Mutex::new(HashMap::new()));

    let state = AppState {
        rabbit,
        broadcast_tx,
        game_scores,
        redis,
    };

    let state = Arc::new(state);
    info!("Application state initialized");

    info!("Configuring CORS policy");
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"]);

    let api_state = state.clone();
    let logger_route = warp::path("api")
        .and(warp::path("logger"))
        .and(warp::path("send"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(api_state.clone()))
        .and_then(logger::send_message);

    let workers_route = warp::path("api")
        .and(warp::path("workers"))
        .and(warp::path("submit"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(api_state.clone()))
        .and_then(workers::submit_number);

    let game_click_route = warp::path("api")
        .and(warp::path("game"))
        .and(warp::path("click"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(api_state.clone()))
        .and_then(game::handle_click);

    let game_scores_route = warp::path("api")
        .and(warp::path("game"))
        .and(warp::path("scores"))
        .and(warp::get())
        .and(with_state(api_state.clone()))
        .and_then(game::get_scores);

    let rpc_route = warp::path("api")
        .and(warp::path("rpc"))
        .and(warp::path("status"))
        .and(warp::post())
        .and(with_state(api_state.clone()))
        .and_then(rpc::check_status);

    let simulator_route = warp::path("api")
        .and(warp::path("simulator"))
        .and(warp::path("simulate"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(api_state.clone()))
        .and_then(simulator::simulate_message);

    let queue_stats_route = warp::path("api")
        .and(warp::path("simulator"))
        .and(warp::path("stats"))
        .and(warp::get())
        .and(with_state(api_state.clone()))
        .and_then(simulator::get_queue_stats);

    let drawing_event_route = warp::path("api")
        .and(warp::path("drawing"))
        .and(warp::path("event"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(api_state.clone()))
        .and_then(collaborative_drawing::handle_drawing_event);

    let cursor_move_route = warp::path("api")
        .and(warp::path("drawing"))
        .and(warp::path("cursor"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(api_state.clone()))
        .and_then(collaborative_drawing::handle_cursor_move);

    let clear_canvas_route = warp::path("api")
        .and(warp::path("drawing"))
        .and(warp::path("clear"))
        .and(warp::post())
        .and(with_state(api_state.clone()))
        .and_then(collaborative_drawing::clear_canvas);

    let load_canvas_route = warp::path("api")
        .and(warp::path("drawing"))
        .and(warp::path("load"))
        .and(warp::get())
        .and(with_state(api_state.clone()))
        .and_then(collaborative_drawing::load_canvas_state);

    let delete_strokes_route = warp::path("api")
        .and(warp::path("drawing"))
        .and(warp::path("delete"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(api_state.clone()))
        .and_then(collaborative_drawing::delete_user_strokes);

    let realtime_drawing_route = warp::path("api")
        .and(warp::path("drawing"))
        .and(warp::path("realtime"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(api_state.clone()))
        .and_then(collaborative_drawing::handle_realtime_drawing_event);

    let save_path_route = warp::path("api")
        .and(warp::path("drawing"))
        .and(warp::path("save-path"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(api_state.clone()))
        .and_then(collaborative_drawing::handle_save_path);


    let ws_state = state.clone();
    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .and(with_state(ws_state))
        .map(|ws: warp::ws::Ws, state: Arc<AppState>| {
            ws.on_upgrade(move |socket| handle_websocket(socket, state))
        });

    info!("Setting up API routes");
    let routes = logger_route
        .or(workers_route)
        .or(game_click_route)
        .or(game_scores_route)
        .or(rpc_route)
        .or(simulator_route)
        .or(queue_stats_route)
        .or(drawing_event_route)
        .or(cursor_move_route)
        .or(clear_canvas_route)
        .or(load_canvas_route)
        .or(delete_strokes_route)
        .or(realtime_drawing_route)
        .or(save_path_route)
        .or(websocket_route)
        .with(cors);

    info!("All routes configured successfully");
    info!("Server starting on http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_state(state: Arc<AppState>) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn handle_websocket(ws: WebSocket, state: Arc<AppState>) {
    use futures_util::{SinkExt, StreamExt};
    use warp::ws::Message;

    info!("New WebSocket connection established");
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut broadcast_rx = state.broadcast_tx.subscribe();

    let broadcast_task = tokio::spawn(async move {
        debug!("Starting WebSocket broadcast task");
        while let Ok(msg) = broadcast_rx.recv().await {
            debug!("Broadcasting message to WebSocket client: {}", msg.demo_type);
            if let Ok(json) = serde_json::to_string(&msg) {
                if ws_tx.send(Message::text(json)).await.is_err() {
                    warn!("Failed to send WebSocket message, client disconnected");
                    break;
                }
            } else {
                error!("Failed to serialize WebSocket message");
            }
        }
        debug!("WebSocket broadcast task ended");
    });

    let receive_task = tokio::spawn(async move {
        debug!("Starting WebSocket receive task");
        while let Some(result) = ws_rx.next().await {
            if let Err(e) = result {
                warn!("WebSocket receive error: {}", e);
                break;
            }
            debug!("Received WebSocket message from client");
        }
        debug!("WebSocket receive task ended");
    });

    tokio::select! {
        _ = broadcast_task => {
            info!("WebSocket broadcast task completed");
        },
        _ = receive_task => {
            info!("WebSocket receive task completed");
        },
    }
    info!("WebSocket connection closed");
}