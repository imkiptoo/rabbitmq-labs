use futures_util::{StreamExt, TryStreamExt};
use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
use serde_json::{json, Value};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Starting RPC Worker...");

    let connection = Connection::connect(
        "amqp://guest:guest@localhost:5672/%2f",
        ConnectionProperties::default(),
    )
    .await?;

    let channel = connection.create_channel().await?;

    // Ensure the RPC queue exists
    channel
        .queue_declare(
            "rpc_requests",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let consumer = channel
        .basic_consume(
            "rpc_requests",
            "rpc_worker",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("RPC Worker listening for requests on 'rpc_requests' queue...");

    let mut stream = consumer.into_stream();
    while let Some(delivery_result) = stream.next().await {
        match delivery_result {
            Ok(delivery) => {
                if let Some(reply_to) = delivery.properties.reply_to() {
                    if let Some(correlation_id) = delivery.properties.correlation_id() {
                        // Parse the request
                        let request: Value = match serde_json::from_slice(&delivery.data) {
                            Ok(req) => req,
                            Err(e) => {
                                eprintln!("Failed to parse request: {}", e);
                                continue;
                            }
                        };

                        println!("Received RPC request: {}", request);

                        // Generate response based on request type
                        let response = match request.get("type").and_then(|t| t.as_str()) {
                            Some("status_check") => {
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
                                json!({
                                    "success": false,
                                    "error": "Unknown request type",
                                    "timestamp": chrono::Utc::now().to_rfc3339()
                                })
                            }
                        };

                        // Send response back
                        let properties = BasicProperties::default()
                            .with_correlation_id(correlation_id.clone());

                        if let Err(e) = channel
                            .basic_publish(
                                "",
                                reply_to.as_str(),
                                BasicPublishOptions::default(),
                                serde_json::to_string(&response)?.as_bytes(),
                                properties,
                            )
                            .await
                        {
                            eprintln!("Failed to send response: {}", e);
                        } else {
                            println!("Sent response to {}", reply_to.as_str());
                        }
                    }
                }

                // Acknowledge the message
                delivery.ack(BasicAckOptions::default()).await?;
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
            }
        }
    }

    Ok(())
}