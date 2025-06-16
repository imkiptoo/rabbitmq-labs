use redis::{Client, RedisResult, AsyncCommands};
use serde_json;
use tracing::{info, error, warn, debug, instrument};

#[derive(Debug)]
pub struct RedisStore {
    client: Client,
}

impl RedisStore {
    #[instrument]
    pub async fn new() -> Result<Self, redis::RedisError> {
        info!("Connecting to Redis server");
        let client = Client::open("redis://127.0.0.1:6379/")
            .map_err(|e| {
                error!("Failed to create Redis client: {}", e);
                e
            })?;
        info!("Redis client created successfully");
        Ok(RedisStore { client })
    }

    #[instrument(skip(self))]
    pub async fn get_connection(&self) -> RedisResult<redis::aio::Connection> {
        debug!("Getting Redis connection");
        self.client.get_async_connection().await
            .map_err(|e| {
                error!("Failed to get Redis connection: {}", e);
                e
            })
    }

    #[instrument(skip(self, drawing_events), fields(events_len = drawing_events.len()))]
    pub async fn save_drawing_state(&self, drawing_events: &str) -> RedisResult<()> {
        debug!("Saving drawing state to Redis");
        let mut conn = self.get_connection().await?;
        let result: redis::RedisResult<()> = conn.set("canvas:drawing_events", drawing_events).await;
        result
            .map_err(|e| {
                error!("Failed to save drawing state: {}", e);
                e
            })
            .map(|_: ()| {
                info!("Drawing state saved successfully");
            })
    }

    #[instrument(skip(self))]
    pub async fn get_drawing_state(&self) -> RedisResult<Option<String>> {
        debug!("Getting drawing state from Redis");
        let mut conn = self.get_connection().await?;
        let result: RedisResult<Option<String>> = conn.get("canvas:drawing_events").await;
        match &result {
            Ok(Some(data)) => info!("Drawing state retrieved, length: {}", data.len()),
            Ok(None) => info!("No drawing state found in Redis"),
            Err(e) => error!("Failed to get drawing state: {}", e),
        }
        result
    }

    #[instrument(skip(self, event_json), fields(event_len = event_json.len()))]
    pub async fn append_drawing_event(&self, event_json: &str) -> RedisResult<()> {
        debug!("Appending drawing event to Redis");
        let mut conn = self.get_connection().await?;
        
        let current_state: Option<String> = conn.get("canvas:drawing_events").await
            .map_err(|e| {
                error!("Failed to get current drawing state: {}", e);
                e
            })?;
        
        let updated_events = match current_state {
            Some(existing) => {
                debug!("Existing drawing state found, length: {}", existing.len());
                if existing.trim().is_empty() || existing == "[]" {
                    format!("[{}]", event_json)
                } else {
                    format!("{},{}]", &existing[..existing.len()-1], event_json)
                }
            },
            None => {
                debug!("No existing drawing state, creating new");
                format!("[{}]", event_json)
            }
        };
        
        let result: redis::RedisResult<()> = conn.set("canvas:drawing_events", &updated_events).await;
        result
            .map_err(|e| {
                error!("Failed to append drawing event: {}", e);
                e
            })
            .map(|_: ()| {
                info!("Drawing event appended successfully");
            })
    }

    #[instrument(skip(self))]
    pub async fn clear_canvas(&self) -> RedisResult<()> {
        info!("Clearing canvas state in Redis");
        let mut conn = self.get_connection().await?;
        let result: redis::RedisResult<()> = conn.set("canvas:drawing_events", "[]").await;
        result
            .map_err(|e| {
                error!("Failed to clear canvas: {}", e);
                e
            })
            .map(|_: ()| {
                info!("Canvas cleared successfully");
            })
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn delete_user_strokes(&self, user_id: &str) -> RedisResult<()> {
        info!("Deleting strokes for user: {}", user_id);
        let mut conn = self.get_connection().await?;
        
        let current_state: Option<String> = conn.get("canvas:drawing_events").await
            .map_err(|e| {
                error!("Failed to get current drawing state for deletion: {}", e);
                e
            })?;
        
        match current_state {
            Some(events_json) => {
                debug!("Found existing drawing events, parsing for user filtering");
                if let Ok(events) = serde_json::from_str::<Vec<serde_json::Value>>(&events_json) {
                    let original_count = events.len();
                    let filtered_events: Vec<serde_json::Value> = events
                        .into_iter()
                        .filter(|event| {
                            if let Some(event_user_id) = event.get("user_id").and_then(|v| v.as_str()) {
                                event_user_id != user_id
                            } else {
                                true
                            }
                        })
                        .collect();
                    
                    let filtered_count = filtered_events.len();
                    let deleted_count = original_count - filtered_count;
                    info!("Filtered {} events, deleted {} events for user {}", filtered_count, deleted_count, user_id);
                    
                    let filtered_json = serde_json::to_string(&filtered_events)
                        .unwrap_or_else(|e| {
                            warn!("Failed to serialize filtered events, using empty array: {}", e);
                            "[]".to_string()
                        });
                    
                    let result: redis::RedisResult<()> = conn.set("canvas:drawing_events", &filtered_json).await;
                    result
                        .map_err(|e| {
                            error!("Failed to save filtered events: {}", e);
                            e
                        })
                        .map(|_: ()| {
                            info!("User strokes deleted successfully for user: {}", user_id);
                        })
                } else {
                    warn!("Failed to parse drawing events JSON, clearing canvas");
                    let result: redis::RedisResult<()> = conn.set("canvas:drawing_events", "[]").await;
                    result
                        .map_err(|e| {
                            error!("Failed to clear canvas after parse error: {}", e);
                            e
                        })
                }
            },
            None => {
                debug!("No existing drawing events found, nothing to delete for user: {}", user_id);
                Ok(())
            }
        }
    }
}