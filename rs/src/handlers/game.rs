use std::sync::Arc;
use warp::{Reply, Rejection};
use serde::{Deserialize, Serialize};
use crate::{AppState, WebSocketMessage};

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

pub async fn handle_click(
    req: ClickRequest,
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    let mut scores = state.game_scores.lock().await;
    let current_score = scores.entry(req.player_name.clone()).or_insert(0);
    *current_score += 1;
    let new_score = *current_score;
    drop(scores);

    let score_update = serde_json::json!({
        "player": req.player_name,
        "score": new_score
    });

    if let Err(e) = state.rabbit.publish_to_exchange("game_scores", &score_update.to_string()).await {
        eprintln!("Failed to publish score update: {}", e);
    }

    let ws_msg = WebSocketMessage {
        demo_type: "game".to_string(),
        data: serde_json::json!({
            "type": "score_update",
            "player": req.player_name,
            "score": new_score
        }),
    };

    let _ = state.broadcast_tx.send(ws_msg);

    if new_score >= 100 {
        let winner_msg = WebSocketMessage {
            demo_type: "game".to_string(),
            data: serde_json::json!({
                "type": "winner",
                "player": req.player_name,
                "score": new_score
            }),
        };
        let _ = state.broadcast_tx.send(winner_msg);
    }

    Ok(warp::reply::json(&ClickResponse {
        success: true,
        score: new_score,
    }))
}

pub async fn get_scores(
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    let scores = state.game_scores.lock().await;
    Ok(warp::reply::json(&ScoresResponse {
        scores: scores.clone(),
    }))
}