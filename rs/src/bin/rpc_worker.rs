use futures_util::{StreamExt, TryStreamExt};
use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
use serde_json::{json, Value};
use std::error::Error;
use tracing::{info, error, warn, debug};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt::init();
    info!("Starting RPC Worker");

    info!("Connecting to RabbitMQ");
    let connection = Connection::connect(
        "amqp://guest:guest@localhost:5672/%2f",
        ConnectionProperties::default(),
    )
    .await
    .map_err(|e| {
        error!("Failed to connect to RabbitMQ: {}", e);
        e
    })?;

    info!("Creating RabbitMQ channel");
    let channel = connection.create_channel().await
        .map_err(|e| {
            error!("Failed to create channel: {}", e);
            e
        })?;

    info!("Declaring RPC requests queue");
    channel
        .queue_declare(
            "rpc_requests",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| {
            error!("Failed to declare rpc_requests queue: {}", e);
            e
        })?;

    info!("Creating consumer for RPC requests");
    let consumer = channel
        .basic_consume(
            "rpc_requests",
            "rpc_worker",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| {
            error!("Failed to create consumer: {}", e);
            e
        })?;

    info!("RPC Worker ready - listening for requests on 'rpc_requests' queue");

    let mut stream = consumer.into_stream();
    debug!("Starting message processing loop");
    
    while let Some(delivery_result) = stream.next().await {
        match delivery_result {
            Ok(delivery) => {
                debug!("Received RPC message");
                
                if let Some(reply_to) = delivery.properties.reply_to() {
                    if let Some(correlation_id) = delivery.properties.correlation_id() {
                        info!("Processing RPC request with correlation_id: {}", correlation_id.as_str());
                        
                        let request: Value = match serde_json::from_slice(&delivery.data) {
                            Ok(req) => {
                                debug!("Successfully parsed RPC request: {}", req);
                                req
                            },
                            Err(e) => {
                                error!("Failed to parse RPC request: {}", e);
                                // Still acknowledge the message to remove it from queue
                                if let Err(ack_err) = delivery.ack(BasicAckOptions::default()).await {
                                    error!("Failed to ack malformed message: {}", ack_err);
                                }
                                continue;
                            }
                        };

                        let request_type = request.get("type").and_then(|t| t.as_str()).unwrap_or("unknown");
                        info!("Processing RPC request type: {}", request_type);

                        let response = match request_type {
                            "status_check" => {
                                info!("Handling status check request");
                                json!({
                                    "success": true,
                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                    "status": "All systems operational",
                                    "server_info": "RabbitMQ Demo Server v1.0",
                                    "uptime": "Running smoothly",
                                    "queue_status": "Active",
                                    "processed_at": chrono::Utc::now().to_rfc3339()
                                })
                            }
                            _ => {
                                warn!("Unknown RPC request type: {}", request_type);
                                json!({
                                    "success": false,
                                    "error": "Unknown request type",
                                    "timestamp": chrono::Utc::now().to_rfc3339()
                                })
                            }
                        };

                        debug!("Generated RPC response: {}", response);

                        let properties = BasicProperties::default()
                            .with_correlation_id(correlation_id.clone());

                        match channel
                            .basic_publish(
                                "",
                                reply_to.as_str(),
                                BasicPublishOptions::default(),
                                serde_json::to_string(&response)
                                    .map_err(|e| {
                                        error!("Failed to serialize RPC response: {}", e);
                                        e
                                    })?
                                    .as_bytes(),
                                properties,
                            )
                            .await
                        {
                            Ok(_) => {
                                info!("Successfully sent RPC response to {} with correlation_id: {}", 
                                     reply_to.as_str(), correlation_id.as_str());
                            }
                            Err(e) => {
                                error!("Failed to send RPC response to {}: {}", reply_to.as_str(), e);
                            }
                        }
                    } else {
                        warn!("Received RPC request without correlation_id");
                    }
                } else {
                    warn!("Received RPC request without reply_to address");
                }

                if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                    error!("Failed to acknowledge RPC message: {}", e);
                } else {
                    debug!("RPC message acknowledged successfully");
                }
            }
            Err(e) => {
                error!("Error receiving RPC message: {}", e);
                break;
            }
        }
    }
    
    warn!("RPC Worker message processing loop ended");

    Ok(())
}