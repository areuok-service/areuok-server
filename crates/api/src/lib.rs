use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use models::{Device, DeviceRegisterRequest, DeviceStatusResponse, DeviceUpdateNameRequest};
use sqlx::PgPool;
use uuid::Uuid;

pub type DbPool = PgPool;

mod error;
mod handlers;

pub use error::AppError;

pub type ApiPool = DbPool;

pub fn create_router(pool: DbPool) -> Router {
    Router::new()
        .route("/devices/register", post(register_device))
        .route("/devices/:id", get(get_device))
        .route(
            "/devices/:id/name",
            axum::routing::patch(update_device_name),
        )
        .route("/search/devices", get(search_devices))
        .route(
            "/devices/:id/signin",
            post(handlers::signin::signin_handler),
        )
        .route("/devices/:id/status", get(get_device_status))
        .route(
            "/supervision/request",
            post(handlers::supervision::create_supervision_request),
        )
        .route(
            "/supervision/pending/:id",
            get(handlers::supervision::pending_requests),
        )
        .route(
            "/supervision/accept",
            post(handlers::supervision::accept_supervision),
        )
        .route(
            "/supervision/reject",
            post(handlers::supervision::reject_supervision),
        )
        .route(
            "/supervision/list/:id",
            get(handlers::supervision::list_supervision_relations),
        )
        .route(
            "/supervision/:relation_id",
            axum::routing::delete(handlers::supervision::remove_supervision),
        )
        .with_state(pool)
}

async fn register_device(
    State(pool): State<DbPool>,
    Json(req): Json<DeviceRegisterRequest>,
) -> Result<Json<Device>, AppError> {
    let existing_name = sqlx::query!(
        r#"
        SELECT device_id
        FROM devices
        WHERE device_name = $1
        "#,
        req.device_name
    )
    .fetch_optional(&pool)
    .await?;

    if existing_name.is_some() {
        return Err(AppError::BadRequest(
            "Device name already exists".to_string(),
        ));
    }

    if let Some(imei) = &req.imei {
        let existing_device = sqlx::query!(
            r#"
            SELECT device_id, device_name, imei, last_name_updated_at
            FROM devices
            WHERE imei = $1
            "#,
            imei
        )
        .fetch_optional(&pool)
        .await?;

        if let Some(existing) = existing_device {
            let device = sqlx::query_as!(
                Device,
                r#"
                SELECT device_id, device_name, imei, mode as "mode: models::DeviceMode", created_at, last_seen_at, last_name_updated_at
                FROM devices
                WHERE device_id = $1
                "#,
                existing.device_id
            )
            .fetch_one(&pool)
            .await?;

            return Ok(Json(device));
        }
    }

    let device_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO devices (device_id, device_name, imei, mode)
        VALUES ($1, $2, $3, $4)
        RETURNING device_id, device_name, imei, mode as "mode: models::DeviceMode", created_at, last_seen_at, last_name_updated_at
        "#,
        device_id,
        req.device_name,
        req.imei,
        req.mode as models::DeviceMode
    )
    .fetch_one(&pool)
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
    .fetch_one(&pool)
    .await?;

    Ok(Json(device))
}

async fn get_device(
    State(pool): State<DbPool>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<Device>, AppError> {
    let device = sqlx::query_as!(
        Device,
        r#"
        SELECT device_id, device_name, imei, mode as "mode: models::DeviceMode", created_at, last_seen_at, last_name_updated_at
        FROM devices
        WHERE device_id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound("Device not found".to_string()))?;

    Ok(Json(device))
}

async fn update_device_name(
    State(pool): State<DbPool>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(req): Json<DeviceUpdateNameRequest>,
) -> Result<Json<Device>, AppError> {
    let current_device = sqlx::query!(
        r#"
        SELECT device_id, device_name, last_name_updated_at
        FROM devices
        WHERE device_id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound("Device not found".to_string()))?;

    let existing_name = sqlx::query!(
        r#"
        SELECT device_id
        FROM devices
        WHERE device_name = $1 AND device_id != $2
        "#,
        req.device_name,
        id
    )
    .fetch_optional(&pool)
    .await?;

    if existing_name.is_some() {
        return Err(AppError::BadRequest(
            "Device name already exists".to_string(),
        ));
    }

    if let Some(last_updated) = current_device.last_name_updated_at {
        let now = Utc::now();
        let time_since_update = now.signed_duration_since(last_updated);

        if time_since_update.num_days() < 15 {
            return Err(AppError::BadRequest(
                format!("Device name cannot be updated. Last updated {} days ago. Minimum 15 days required.", time_since_update.num_days())
            ));
        }
    }

    sqlx::query!(
        r#"
        UPDATE devices
        SET device_name = $1,
            last_name_updated_at = NOW()
        WHERE device_id = $2
        RETURNING device_id, device_name, imei, mode as "mode: models::DeviceMode", created_at, last_seen_at, last_name_updated_at
        "#,
        req.device_name,
        id
    )
    .fetch_one(&pool)
    .await?;

    let device = sqlx::query_as!(
        Device,
        r#"
        SELECT device_id, device_name, imei, mode as "mode: models::DeviceMode", created_at, last_seen_at, last_name_updated_at
        FROM devices
        WHERE device_id = $1
        "#,
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(device))
}

async fn get_device_status(
    State(pool): State<DbPool>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<DeviceStatusResponse>, AppError> {
    let device = sqlx::query!(
        r#"
        SELECT device_id, device_name, mode as "mode: models::DeviceMode"
        FROM devices
        WHERE device_id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound("Device not found".to_string()))?;

    let last_signin = sqlx::query!(
        r#"
        SELECT date
        FROM signin_records
        WHERE device_id = $1
        ORDER BY date DESC
        LIMIT 1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await?;

    let streak = sqlx::query!(
        r#"
        SELECT streak
        FROM signin_records
        WHERE device_id = $1
        ORDER BY date DESC
        LIMIT 1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await?;

    Ok(Json(DeviceStatusResponse {
        device_id: device.device_id,
        device_name: device.device_name,
        mode: device.mode,
        last_signin: last_signin.map(|r| r.date),
        streak: streak.map(|r| r.streak).unwrap_or(0),
    }))
}

async fn search_devices(
    State(pool): State<DbPool>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<Device>>, AppError> {
    let query = params.get("q").map(|v| v.as_str()).unwrap_or("");

    if query.is_empty() || query.len() < 2 {
        return Ok(Json(vec![]));
    }

    let search_pattern = format!("%{}%", query);

    let devices = sqlx::query_as!(
        Device,
        r#"
        SELECT device_id, device_name, imei, mode as "mode: models::DeviceMode", created_at, last_seen_at, last_name_updated_at
        FROM devices
        WHERE device_name ILIKE $1
        ORDER BY device_name
        LIMIT 20
        "#,
        search_pattern
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(devices))
}
