use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "device_mode", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DeviceMode {
    Signin,
    Supervisor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub device_id: Uuid,
    pub device_name: String,
    pub imei: Option<String>,
    pub mode: DeviceMode,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub last_name_updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRegisterRequest {
    pub device_name: String,
    pub imei: Option<String>,
    pub mode: DeviceMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceUpdateNameRequest {
    pub device_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "supervision_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SupervisionStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisionRequest {
    pub request_id: Uuid,
    pub supervisor_id: Uuid,
    pub target_id: Uuid,
    pub status: SupervisionStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisionCreateRequest {
    pub supervisor_id: Uuid,
    pub target_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisionRelation {
    pub relation_id: Uuid,
    pub supervisor_id: Uuid,
    pub target_id: Uuid,
    pub supervisor_name: Option<String>,
    pub target_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigninRecord {
    pub device_id: Uuid,
    pub date: DateTime<Utc>,
    pub streak: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatusResponse {
    pub device_id: Uuid,
    pub device_name: String,
    pub mode: DeviceMode,
    pub last_signin: Option<DateTime<Utc>>,
    pub streak: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum SseEvent {
    #[serde(rename = "signin")]
    Signin {
        device_id: Uuid,
        device_name: String,
        time: DateTime<Utc>,
    },
}
