use crate::error::AppError;
use crate::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use models::{Device, SigninRecord, SseEvent};
use uuid::Uuid;

pub async fn signin_handler(
    State(state): State<AppState>,
    Path(device_id): Path<Uuid>,
) -> Result<Json<models::SigninRecord>, AppError> {
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        UPDATE devices
        SET last_seen_at = NOW()
        WHERE device_id = $1
        "#,
        device_id
    )
    .execute(&state.pool)
    .await?;

    let existing_today = sqlx::query!(
        r#"
        SELECT id, device_id, date, streak
        FROM signin_records
        WHERE device_id = $1 AND date::date = NOW()::date
        "#,
        device_id
    )
    .fetch_optional(&state.pool)
    .await?;

    if let Some(record) = existing_today {
        return Ok(Json(SigninRecord {
            device_id,
            date: record.date,
            streak: record.streak,
        }));
    }

    let yesterday = now - chrono::Duration::days(1);

    let last_record = sqlx::query!(
        r#"
        SELECT id, device_id, date, streak
        FROM signin_records
        WHERE device_id = $1
        ORDER BY date DESC
        LIMIT 1
        "#,
        device_id
    )
    .fetch_optional(&state.pool)
    .await?;

    let streak = if let Some(record) = last_record {
        if record.date.date_naive() == yesterday.date_naive() {
            record.streak + 1
        } else {
            1
        }
    } else {
        1
    };

    sqlx::query!(
        r#"
        INSERT INTO signin_records (device_id, date, streak)
        VALUES ($1, $2, $3)
        "#,
        device_id,
        now,
        streak
    )
    .execute(&state.pool)
    .await?;

    let device = sqlx::query_as!(
        Device,
        r#"
        SELECT device_id, device_name, imei, mode as "mode: models::DeviceMode", created_at, last_seen_at, last_name_updated_at
        FROM devices
        WHERE device_id = $1
        "#,
        device_id
    )
    .fetch_one(&state.pool)
    .await?;

    let event = SseEvent::Signin {
        device_id,
        device_name: device.device_name,
        time: now,
    };

    let _ = state.sse_manager.broadcast(event).await;

    Ok(Json(SigninRecord {
        device_id,
        date: now,
        streak,
    }))
}
