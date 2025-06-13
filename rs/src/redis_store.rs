use redis::{Client, RedisResult, AsyncCommands};
use serde_json;

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

    pub async fn delete_user_strokes(&self, user_id: &str) -> RedisResult<()> {
        let mut conn = self.get_connection().await?;
        
        let current_state: Option<String> = conn.get("canvas:drawing_events").await?;
        
        match current_state {
            Some(events_json) => {
                // Parse the JSON array of events
                if let Ok(events) = serde_json::from_str::<Vec<serde_json::Value>>(&events_json) {
                    // Filter out events from the specified user
                    let filtered_events: Vec<serde_json::Value> = events
                        .into_iter()
                        .filter(|event| {
                            if let Some(event_user_id) = event.get("user_id").and_then(|v| v.as_str()) {
                                event_user_id != user_id
                            } else {
                                true // Keep events without user_id
                            }
                        })
                        .collect();
                    
                    // Save the filtered events back to Redis
                    let filtered_json = serde_json::to_string(&filtered_events).unwrap_or_else(|_| "[]".to_string());
                    conn.set("canvas:drawing_events", &filtered_json).await
                } else {
                    // If parsing fails, clear the canvas
                    conn.set("canvas:drawing_events", "[]").await
                }
            },
            None => Ok(()) // Nothing to delete
        }
    }
}