use futures_util::TryStreamExt;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    Consumer, ExchangeKind,
};
use serde_json::Value;
use uuid::Uuid;
use tracing::{info, error, warn, debug, instrument};

#[derive(Debug)]
pub struct RabbitMQConnection {
    pub connection: Connection,
    pub channel: Channel,
}

impl RabbitMQConnection {
    #[instrument]
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        info!("Connecting to RabbitMQ server");
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
                error!("Failed to create RabbitMQ channel: {}", e);
                e
            })?;

        let rabbit = Self {
            connection,
            channel,
        };

        info!("Setting up queues and exchanges");
        rabbit.setup_queues_and_exchanges().await?;
        info!("RabbitMQ connection initialized successfully");
        Ok(rabbit)
    }

    #[instrument(skip(self))]
    async fn setup_queues_and_exchanges(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Declaring queue: message_logger");
        self.channel
            .queue_declare(
                "message_logger",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to declare message_logger queue: {}", e);
                e
            })?;

        info!("Declaring queue: number_doubler");
        self.channel
            .queue_declare(
                "number_doubler",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to declare number_doubler queue: {}", e);
                e
            })?;

        info!("Declaring exchange: game_scores (fanout)");
        self.channel
            .exchange_declare(
                "game_scores",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to declare game_scores exchange: {}", e);
                e
            })?;

        info!("Declaring queue: rpc_requests");
        self.channel
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

        info!("Declaring queue: rpc_replies");
        self.channel
            .queue_declare(
                "rpc_replies",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to declare rpc_replies queue: {}", e);
                e
            })?;

        info!("Declaring exchange: drawing_fanout (fanout)");
        self.channel
            .exchange_declare(
                "drawing_fanout",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to declare drawing_fanout exchange: {}", e);
                e
            })?;

        info!("Declaring exchange: collaborative_drawing (fanout)");
        self.channel
            .exchange_declare(
                "collaborative_drawing",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to declare collaborative_drawing exchange: {}", e);
                e
            })?;

        info!("All queues and exchanges declared successfully");
        Ok(())
    }

    #[instrument(skip(self, message), fields(queue = %queue, message_len = message.len()))]
    pub async fn publish_message(
        &self,
        queue: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("Publishing message to queue: {}", queue);
        self.channel
            .basic_publish(
                "",
                queue,
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to publish message to queue {}: {}", queue, e);
                e
            })?
            .await
            .map_err(|e| {
                error!("Failed to confirm message publication to queue {}: {}", queue, e);
                e
            })?;
        info!("Message published successfully to queue: {}", queue);
        Ok(())
    }

    #[instrument(skip(self, message), fields(exchange = %exchange, message_len = message.len()))]
    pub async fn publish_to_exchange(
        &self,
        exchange: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("Publishing message to exchange: {}", exchange);
        self.channel
            .basic_publish(
                exchange,
                "",
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to publish message to exchange {}: {}", exchange, e);
                e
            })?
            .await
            .map_err(|e| {
                error!("Failed to confirm message publication to exchange {}: {}", exchange, e);
                e
            })?;
        info!("Message published successfully to exchange: {}", exchange);
        Ok(())
    }

    #[instrument(skip(self, message), fields(exchange = %exchange, message_len = message.len()))]
    pub async fn publish_fanout(
        &self,
        exchange: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("Publishing fanout message to exchange: {}", exchange);
        self.channel
            .basic_publish(
                exchange,
                "",
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to publish fanout message to exchange {}: {}", exchange, e);
                e
            })?
            .await
            .map_err(|e| {
                error!("Failed to confirm fanout message publication to exchange {}: {}", exchange, e);
                e
            })?;
        info!("Fanout message published successfully to exchange: {}", exchange);
        Ok(())
    }

    #[instrument(skip(self), fields(queue = %queue))]
    pub async fn consume_queue(
        &self,
        queue: &str,
    ) -> Result<Consumer, Box<dyn std::error::Error + Send + Sync>> {
        info!("Starting consumer for queue: {}", queue);
        let consumer = self
            .channel
            .basic_consume(
                queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to create consumer for queue {}: {}", queue, e);
                e
            })?;
        info!("Consumer created successfully for queue: {}", queue);
        Ok(consumer)
    }

    #[instrument(skip(self, request_data), fields(correlation_id))]
    pub async fn rpc_call(
        &self,
        request_data: Value,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let correlation_id = Uuid::new_v4().to_string();
        let reply_queue = format!("rpc_reply_{}", correlation_id);
        
        tracing::Span::current().record("correlation_id", &correlation_id);
        info!("Starting RPC call with correlation_id: {}", correlation_id);

        debug!("Declaring temporary reply queue: {}", reply_queue);
        self.channel
            .queue_declare(
                &reply_queue,
                QueueDeclareOptions {
                    exclusive: true,
                    auto_delete: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to declare reply queue {}: {}", reply_queue, e);
                e
            })?;

        let mut properties = BasicProperties::default();
        properties = properties.with_correlation_id(correlation_id.clone().into());
        properties = properties.with_reply_to(reply_queue.clone().into());

        debug!("Publishing RPC request with correlation_id: {}", correlation_id);
        self.channel
            .basic_publish(
                "",
                "rpc_requests",
                BasicPublishOptions::default(),
                serde_json::to_string(&request_data)
                    .map_err(|e| {
                        error!("Failed to serialize RPC request: {}", e);
                        e
                    })?
                    .as_bytes(),
                properties,
            )
            .await
            .map_err(|e| {
                error!("Failed to publish RPC request: {}", e);
                e
            })?;

        debug!("Waiting for RPC response on queue: {}", reply_queue);
        let consumer = self
            .channel
            .basic_consume(
                &reply_queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to create RPC reply consumer: {}", e);
                e
            })?;

        use futures_util::StreamExt;
        let mut stream = consumer.into_stream();
        if let Some(delivery_result) = stream.next().await {
            match delivery_result {
                Ok(delivery) => {
                    debug!("Received RPC response for correlation_id: {}", correlation_id);
                    let response: Value = serde_json::from_slice(&delivery.data)
                        .map_err(|e| {
                            error!("Failed to deserialize RPC response: {}", e);
                            e
                        })?;
                    
                    delivery.ack(BasicAckOptions::default()).await
                        .map_err(|e| {
                            warn!("Failed to ack RPC response: {}", e);
                            e
                        })?;

                    self.channel
                        .queue_delete(&reply_queue, QueueDeleteOptions::default())
                        .await
                        .map_err(|e| {
                            warn!("Failed to delete reply queue {}: {}", reply_queue, e);
                            e
                        })?;

                    info!("RPC call completed successfully for correlation_id: {}", correlation_id);
                    return Ok(response);
                }
                Err(e) => {
                    error!("RPC delivery error for correlation_id {}: {}", correlation_id, e);
                    let _ = self.channel
                        .queue_delete(&reply_queue, QueueDeleteOptions::default())
                        .await;
                    return Err(format!("RPC delivery error: {}", e).into());
                }
            }
        }

        warn!("RPC timeout for correlation_id: {}", correlation_id);
        let _ = self.channel
            .queue_delete(&reply_queue, QueueDeleteOptions::default())
            .await;
        Err("RPC timeout".into())
    }
}
