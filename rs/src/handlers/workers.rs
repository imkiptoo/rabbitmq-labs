use std::sync::Arc;
use warp::{Reply, Rejection};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use futures_util::StreamExt;
use crate::{AppState, WebSocketMessage};
use tracing::{info, error, warn, debug, instrument};

#[derive(Deserialize)]
pub struct NumberRequest {
    pub number: i32,
}

#[derive(Serialize)]
pub struct NumberResponse {
    pub success: bool,
    pub message: String,
}

#[instrument(skip(req, state), fields(number = req.number))]
pub async fn submit_number(
    req: NumberRequest,
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    let task_id = uuid::Uuid::new_v4().to_string();
    info!("Submitting number {} for processing with task_id: {}", req.number, task_id);
    
    let task_data = serde_json::json!({
        "number": req.number,
        "task_id": task_id
    });

    match state.rabbit.publish_message("number_doubler", &task_data.to_string()).await {
        Ok(_) => {
            info!("Number {} queued successfully with task_id: {}", req.number, task_id);
            tokio::spawn(start_workers(state.clone()));
            debug!("Workers started for processing");
            
            Ok(warp::reply::json(&NumberResponse {
                success: true,
                message: "Number submitted for processing".to_string(),
            }))
        }
        Err(e) => {
            error!("Failed to submit number {} for processing: {}", req.number, e);
            Ok(warp::reply::json(&NumberResponse {
                success: false,
                message: format!("Failed to submit number: {}", e),
            }))
        }
    }
}

#[instrument(skip(state))]
async fn start_workers(state: Arc<AppState>) {
    info!("Starting worker processes");
    for worker_id in 1..=3 {
        let state_clone = state.clone();
        tokio::spawn(async move {
            info!("Starting worker {}", worker_id);
            worker_process(worker_id, state_clone).await;
        });
    }
    info!("All worker processes started");
}

#[instrument(skip(state), fields(worker_id = worker_id))]
async fn worker_process(worker_id: u8, state: Arc<AppState>) {
    info!("Worker {} starting processing loop", worker_id);
    
    match state.rabbit.consume_queue("number_doubler").await {
        Ok(consumer) => {
            info!("Worker {} connected to queue consumer", worker_id);
            let mut stream = consumer;
            
            while let Some(delivery_result) = stream.next().await {
                match delivery_result {
                    Ok(delivery) => {
                        debug!("Worker {} received message", worker_id);
                        
                        match String::from_utf8(delivery.data.clone()) {
                            Ok(task_str) => {
                                match serde_json::from_str::<serde_json::Value>(&task_str) {
                                    Ok(task_data) => {
                                        if let Some(number) = task_data["number"].as_i64() {
                                            let task_id = task_data["task_id"].as_str().unwrap_or("unknown");
                                            info!("Worker {} processing task {} with number {}", worker_id, task_id, number);
                                            
                                            let delay = rand::random::<u64>() % 3000 + 1000;
                                            debug!("Worker {} simulating processing delay of {}ms", worker_id, delay);
                                            sleep(Duration::from_millis(delay)).await;
                                            
                                            let result = number * 2;
                                            info!("Worker {} completed task {} ({}*2={})", worker_id, task_id, number, result);
                                            
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
                                            
                                            if let Err(_) = state.broadcast_tx.send(ws_msg) {
                                                warn!("Worker {} could not broadcast result - no WebSocket clients", worker_id);
                                            } else {
                                                debug!("Worker {} broadcasted result to WebSocket clients", worker_id);
                                            }
                                        } else {
                                            warn!("Worker {} received task without valid number field", worker_id);
                                        }
                                    }
                                    Err(e) => {
                                        error!("Worker {} failed to parse task JSON: {}", worker_id, e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Worker {} received invalid UTF-8 message: {}", worker_id, e);
                            }
                        }
                        
                        if let Err(e) = delivery.ack(lapin::options::BasicAckOptions::default()).await {
                            error!("Worker {} failed to ack message: {}", worker_id, e);
                        } else {
                            debug!("Worker {} acknowledged message", worker_id);
                        }
                    }
                    Err(e) => {
                        error!("Worker {} delivery error: {}", worker_id, e);
                        break;
                    }
                }
            }
            warn!("Worker {} processing loop ended", worker_id);
        }
        Err(e) => {
            error!("Worker {} failed to connect to queue consumer: {}", worker_id, e);
        }
    }
}