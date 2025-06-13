use redis::{Client, RedisResult, AsyncCommands};

#[derive(Debug)]
pub struct RedisStore {
    client: Client,
}

impl RedisStore {
    pub async fn new() -> Result<Self, redis::RedisError> {
        let client = Client::open("redis://127.0.0.1:6379/")?;
        Ok(RedisStore { client })
    }

    pub async fn get_connection(&self) -> RedisResult<redis::aio::Connection> {
        self.client.get_async_connection().await
    }

    pub async fn save_drawing_state(&self, drawing_events: &str) -> RedisResult<()> {
        let mut conn = self.get_connection().await?;
        conn.set("canvas:drawing_events", drawing_events).await
    }

    pub async fn get_drawing_state(&self) -> RedisResult<Option<String>> {
        let mut conn = self.get_connection().await?;
        conn.get("canvas:drawing_events").await
    }

    pub async fn append_drawing_event(&self, event_json: &str) -> RedisResult<()> {
        let mut conn = self.get_connection().await?;
        
        let current_state: Option<String> = conn.get("canvas:drawing_events").await?;
        
        let updated_events = match current_state {
            Some(existing) => {
                if existing.trim().is_empty() || existing == "[]" {
                    format!("[{}]", event_json)
                } else {
                    format!("{},{}]", &existing[..existing.len()-1], event_json)
                }
            },
            None => format!("[{}]", event_json)
        };
        
        conn.set("canvas:drawing_events", &updated_events).await
    }

    pub async fn clear_canvas(&self) -> RedisResult<()> {
        let mut conn = self.get_connection().await?;
        conn.set("canvas:drawing_events", "[]").await
    }
}