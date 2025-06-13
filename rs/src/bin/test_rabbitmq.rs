use rabbitmq_demos::rabbitmq::RabbitMQConnection;

#[tokio::main]
async fn main() {
    println!("Testing RabbitMQ connection...");
    
    match RabbitMQConnection::new().await {
        Ok(_) => println!("RabbitMQ connection successful!"),
        Err(e) => println!("RabbitMQ connection failed: {}", e),
    }
}