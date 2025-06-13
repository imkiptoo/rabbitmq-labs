use std::sync::Arc;
use warp::{Reply, Rejection};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use futures_util::StreamExt;
use crate::{AppState, WebSocketMessage};

#[derive(Deserialize)]
pub struct NumberRequest {
    pub number: i32,
}

#[derive(Serialize)]
pub struct NumberResponse {
    pub success: bool,
    pub message: String,
}

pub async fn submit_number(
    req: NumberRequest,
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    let task_data = serde_json::json!({
        "number": req.number,
        "task_id": uuid::Uuid::new_v4().to_string()
    });

    match state.rabbit.publish_message("number_doubler", &task_data.to_string()).await {
        Ok(_) => {
            tokio::spawn(start_workers(state.clone()));
            
            Ok(warp::reply::json(&NumberResponse {
                success: true,
                message: "Number submitted for processing".to_string(),
            }))
        }
        Err(e) => {
            Ok(warp::reply::json(&NumberResponse {
                success: false,
                message: format!("Failed to submit number: {}", e),
            }))
        }
    }
}

async fn start_workers(state: Arc<AppState>) {
    for worker_id in 1..=3 {
        let state_clone = state.clone();
        tokio::spawn(async move {
            worker_process(worker_id, state_clone).await;
        });
    }
}

async fn worker_process(worker_id: u8, state: Arc<AppState>) {
    if let Ok(consumer) = state.rabbit.consume_queue("number_doubler").await {
        let mut stream = consumer;
        while let Some(delivery_result) = stream.next().await {
            match delivery_result {
                Ok(delivery) => {
                    if let Ok(task_str) = String::from_utf8(delivery.data.clone()) {
                        if let Ok(task_data) = serde_json::from_str::<serde_json::Value>(&task_str) {
                            if let Some(number) = task_data["number"].as_i64() {
                                let task_id = task_data["task_id"].as_str().unwrap_or("unknown");
                                
                                let delay = rand::random::<u64>() % 3000 + 1000;
                                sleep(Duration::from_millis(delay)).await;
                                
                                let result = number * 2;
                                
                                let ws_msg = WebSocketMessage {
                                    demo_type: "workers".to_string(),
                                    data: serde_json::json!({
                                        "worker_id": worker_id,
                                        "task_id": task_id,
                                        "original": number,
                                        "result": result,
                                        "processing_time": delay
                                    }),
                                };
                                
                                let _ = state.broadcast_tx.send(ws_msg);
                            }
                        }
                    }
                    
                    let _ = delivery.ack(lapin::options::BasicAckOptions::default()).await;
                }
                Err(e) => {
                    eprintln!("Worker {} error: {}", worker_id, e);
                    break;
                }
            }
        }
    }
}