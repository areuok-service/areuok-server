use axum::{
    extract::{Path, State},
    response::{sse::Event, Sse},
};
use futures::Stream;
use models::SseEvent;
use serde_json;
use sqlx::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct SseManager {
    channels: Arc<Mutex<broadcast::Sender<SseEvent>>>,
}

impl SseManager {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self {
            channels: Arc::new(Mutex::new(tx)),
        }
    }

    pub async fn broadcast(&self, event: SseEvent) -> Result<usize, String> {
        let tx = self.channels.lock().await;
        match tx.send(event) {
            Ok(receiver_count) => Ok(receiver_count),
            Err(_) => Err("No active subscribers".to_string()),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SseEvent> {
        let tx = self.channels.blocking_lock();
        tx.subscribe()
    }
}

impl Default for SseManager {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn sse_handler(
    Path(device_id): Path<Uuid>,
    State(state): State<super::AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let pool = state.pool;
    let mut rx = state.sse_manager.subscribe();

    let stream = async_stream::stream! {
        while let Ok(event) = rx.recv().await {
            if should_send_to_device(&event, device_id, &pool).await {
                match convert_to_sse_event(event) {
                    Ok(sse_event) => yield Ok(sse_event),
                    Err(e) => {
                        log::error!("Failed to convert SSE event: {}", e);
                        let error_json = serde_json::json!({
                            "error": "Failed to process event"
                        });
                        let error_event = Event::default().json_data(error_json).unwrap();
                        yield Ok(error_event);
                    }
                }
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new().interval(std::time::Duration::from_secs(30)),
    )
}

async fn should_send_to_device(event: &SseEvent, device_id: Uuid, pool: &PgPool) -> bool {
    match event {
        SseEvent::Signin {
            device_id: signin_device_id,
            ..
        } => is_supervisor_of(device_id, *signin_device_id, pool).await,
    }
}

async fn is_supervisor_of(supervisor_id: Uuid, target_id: Uuid, pool: &PgPool) -> bool {
    let result = sqlx::query(
        r#"
        SELECT relation_id
        FROM supervision_relations
        WHERE supervisor_id = $1 AND target_id = $2
        "#,
    )
    .bind(supervisor_id)
    .bind(target_id)
    .fetch_optional(pool)
    .await;

    result.ok().flatten().is_some()
}

fn convert_to_sse_event(event: SseEvent) -> Result<Event, String> {
    let json_data =
        serde_json::to_string(&event).map_err(|e| format!("Failed to serialize event: {}", e))?;

    Ok(Event::default()
        .json_data(json_data)
        .map_err(|e| format!("Failed to set JSON data: {}", e))?)
}
