use futures_util::TryStreamExt;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    Consumer, ExchangeKind,
};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug)]
pub struct RabbitMQConnection {
    pub connection: Connection,
    pub channel: Channel,
}

impl RabbitMQConnection {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let connection = Connection::connect(
            "amqp://guest:guest@localhost:5672/%2f",
            ConnectionProperties::default(),
        )
        .await?;

        let channel = connection.create_channel().await?;

        let rabbit = Self {
            connection,
            channel,
        };

        rabbit.setup_queues_and_exchanges().await?;
        Ok(rabbit)
    }

    async fn setup_queues_and_exchanges(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.channel
            .queue_declare(
                "message_logger",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        self.channel
            .queue_declare(
                "number_doubler",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        self.channel
            .exchange_declare(
                "game_scores",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        self.channel
            .queue_declare(
                "rpc_requests",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        self.channel
            .queue_declare(
                "rpc_replies",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        // Drawing board fanout exchange
        self.channel
            .exchange_declare(
                "drawing_fanout",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        // Collaborative drawing fanout exchange
        self.channel
            .exchange_declare(
                "collaborative_drawing",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(())
    }

    pub async fn publish_message(
        &self,
        queue: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.channel
            .basic_publish(
                "",
                queue,
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await?;
        Ok(())
    }

    pub async fn publish_to_exchange(
        &self,
        exchange: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.channel
            .basic_publish(
                exchange,
                "",
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await?;
        Ok(())
    }

    pub async fn publish_fanout(
        &self,
        exchange: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.channel
            .basic_publish(
                exchange,
                "",
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await?;
        Ok(())
    }

    pub async fn consume_queue(
        &self,
        queue: &str,
    ) -> Result<Consumer, Box<dyn std::error::Error + Send + Sync>> {
        let consumer = self
            .channel
            .basic_consume(
                queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        Ok(consumer)
    }

    pub async fn rpc_call(
        &self,
        request_data: Value,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let correlation_id = Uuid::new_v4().to_string();
        let reply_queue = format!("rpc_reply_{}", correlation_id);

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
            .await?;

        let mut properties = BasicProperties::default();
        properties = properties.with_correlation_id(correlation_id.clone().into());
        properties = properties.with_reply_to(reply_queue.clone().into());

        self.channel
            .basic_publish(
                "",
                "rpc_requests",
                BasicPublishOptions::default(),
                serde_json::to_string(&request_data)?.as_bytes(),
                properties,
            )
            .await?;

        let consumer = self
            .channel
            .basic_consume(
                &reply_queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        use futures_util::StreamExt;
        let mut stream = consumer.into_stream();
        if let Some(delivery_result) = stream.next().await {
            match delivery_result {
                Ok(delivery) => {
                    let response: Value = serde_json::from_slice(&delivery.data)?;
                    delivery.ack(BasicAckOptions::default()).await?;

                    self.channel
                        .queue_delete(&reply_queue, QueueDeleteOptions::default())
                        .await?;

                    return Ok(response);
                }
                Err(e) => {
                    self.channel
                        .queue_delete(&reply_queue, QueueDeleteOptions::default())
                        .await?;
                    return Err(format!("RPC delivery error: {}", e).into());
                }
            }
        }

        self.channel
            .queue_delete(&reply_queue, QueueDeleteOptions::default())
            .await?;
        Err("RPC timeout".into())
    }
}
