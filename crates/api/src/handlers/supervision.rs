use crate::error::AppError;
use crate::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use models::{SupervisionCreateRequest, SupervisionRelation};
use uuid::Uuid;

pub async fn create_supervision_request(
    State(state): State<AppState>,
    Json(req): Json<SupervisionCreateRequest>,
) -> Result<Json<models::SupervisionRequest>, AppError> {
    let request_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO supervision_requests (request_id, supervisor_id, target_id, status)
        VALUES ($1, $2, $3, 'pending')
        "#,
        request_id,
        req.supervisor_id,
        req.target_id
    )
    .execute(&state.pool)
    .await?;

    let supervision_request = sqlx::query_as!(
        models::SupervisionRequest,
        r#"
        SELECT request_id, supervisor_id, target_id,
               status as "status: models::SupervisionStatus", created_at
        FROM supervision_requests
        WHERE request_id = $1
        "#,
        request_id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(supervision_request))
}

pub async fn pending_requests(
    State(state): State<AppState>,
    Path(device_id): Path<Uuid>,
) -> Result<Json<Vec<models::SupervisionRequest>>, AppError> {
    let requests = sqlx::query_as!(
        models::SupervisionRequest,
        r#"
        SELECT request_id, supervisor_id, target_id,
               status as "status: models::SupervisionStatus", created_at
        FROM supervision_requests
        WHERE target_id = $1 AND status = 'pending'
        ORDER BY created_at DESC
        "#,
        device_id
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(requests))
}

pub async fn accept_supervision(
    State(state): State<AppState>,
    Json(req): Json<SupervisionCreateRequest>,
) -> Result<Json<()>, AppError> {
    let existing_relation = sqlx::query!(
        r#"
        SELECT relation_id
        FROM supervision_relations
        WHERE supervisor_id = $1 AND target_id = $2
        "#,
        req.supervisor_id,
        req.target_id
    )
    .fetch_optional(&state.pool)
    .await?;

    if existing_relation.is_some() {
        return Err(AppError::BadRequest(
            "Supervision relation already exists".to_string(),
        ));
    }

    let supervision_request = sqlx::query!(
        r#"
        SELECT request_id, supervisor_id, target_id
        FROM supervision_requests
        WHERE supervisor_id = $1 AND target_id = $2 AND status = 'pending'
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        req.supervisor_id,
        req.target_id
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound(
        "Pending supervision request not found".to_string(),
    ))?;

    sqlx::query!(
        r#"
        UPDATE supervision_requests
        SET status = 'accepted'
        WHERE request_id = $1
        "#,
        supervision_request.request_id
    )
    .execute(&state.pool)
    .await?;

    let relation_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO supervision_relations (relation_id, supervisor_id, target_id)
        VALUES ($1, $2, $3)
        "#,
        relation_id,
        supervision_request.supervisor_id,
        supervision_request.target_id
    )
    .execute(&state.pool)
    .await?;

    Ok(Json(()))
}

pub async fn reject_supervision(
    State(state): State<AppState>,
    Json(req): Json<SupervisionCreateRequest>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query!(
        r#"
        UPDATE supervision_requests
        SET status = 'rejected'
        WHERE supervisor_id = $1 AND target_id = $2 AND status = 'pending'
        "#,
        req.supervisor_id,
        req.target_id
    )
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Pending supervision request not found".to_string(),
        ));
    }

    Ok(Json(()))
}

pub async fn list_supervision_relations(
    State(state): State<AppState>,
    Path(device_id): Path<Uuid>,
) -> Result<Json<Vec<SupervisionRelation>>, AppError> {
    let relations = sqlx::query_as!(
        SupervisionRelation,
        r#"
        SELECT sr.relation_id, sr.supervisor_id, sr.target_id, sr.created_at,
               d1.device_name as supervisor_name,
               d2.device_name as target_name
        FROM supervision_relations sr
        LEFT JOIN devices d1 ON sr.supervisor_id = d1.device_id
        LEFT JOIN devices d2 ON sr.target_id = d2.device_id
        WHERE sr.supervisor_id = $1 OR sr.target_id = $1
        ORDER BY sr.created_at DESC
        "#,
        device_id
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(relations))
}

pub async fn remove_supervision(
    State(state): State<AppState>,
    Path(relation_id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    sqlx::query!(
        r#"
        DELETE FROM supervision_relations
        WHERE relation_id = $1
        "#,
        relation_id
    )
    .execute(&state.pool)
    .await?;

    Ok(Json(()))
}
