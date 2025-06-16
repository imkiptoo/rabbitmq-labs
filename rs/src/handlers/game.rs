use std::sync::Arc;
use warp::{Reply, Rejection};
use serde::{Deserialize, Serialize};
use crate::{AppState, WebSocketMessage};
use tracing::{info, error, warn, debug, instrument};

#[derive(Deserialize)]
pub struct ClickRequest {
    pub player_name: String,
}

#[derive(Serialize)]
pub struct ClickResponse {
    pub success: bool,
    pub score: u32,
}

#[derive(Serialize)]
pub struct ScoresResponse {
    pub scores: std::collections::HashMap<String, u32>,
}

#[instrument(skip(req, state), fields(player = %req.player_name))]
pub async fn handle_click(
    req: ClickRequest,
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    info!("Player {} clicked", req.player_name);
    
    let mut scores = state.game_scores.lock().await;
    let current_score = scores.entry(req.player_name.clone()).or_insert(0);
    *current_score += 1;
    let new_score = *current_score;
    drop(scores);
    
    info!("Player {} new score: {}", req.player_name, new_score);

    let score_update = serde_json::json!({
        "player": req.player_name,
        "score": new_score
    });

    if let Err(e) = state.rabbit.publish_to_exchange("game_scores", &score_update.to_string()).await {
        error!("Failed to publish score update for {}: {}", req.player_name, e);
    } else {
        debug!("Score update published to RabbitMQ for player {}", req.player_name);
    }

    let ws_msg = WebSocketMessage {
        demo_type: "game".to_string(),
        data: serde_json::json!({
            "type": "score_update",
            "player": req.player_name,
            "score": new_score
        }),
    };

    if let Err(_) = state.broadcast_tx.send(ws_msg) {
        warn!("No WebSocket clients for game score update");
    } else {
        debug!("Game score update broadcasted to WebSocket clients");
    }

    if new_score >= 100 {
        info!("Player {} reached winning score: {}", req.player_name, new_score);
        let winner_msg = WebSocketMessage {
            demo_type: "game".to_string(),
            data: serde_json::json!({
                "type": "winner",
                "player": req.player_name,
                "score": new_score
            }),
        };
        if let Err(_) = state.broadcast_tx.send(winner_msg) {
            warn!("No WebSocket clients for winner announcement");
        } else {
            info!("Winner announcement broadcasted for player {}", req.player_name);
        }
    }

    Ok(warp::reply::json(&ClickResponse {
        success: true,
        score: new_score,
    }))
}

#[instrument(skip(state))]
pub async fn get_scores(
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    debug!("Getting current game scores");
    let scores = state.game_scores.lock().await;
    let score_count = scores.len();
    info!("Retrieved scores for {} players", score_count);
    Ok(warp::reply::json(&ScoresResponse {
        scores: scores.clone(),
    }))
}