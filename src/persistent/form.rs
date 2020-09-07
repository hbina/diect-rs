use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PersistentValueValidityRequest {
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct PersistentValueValidityResponse {
    pub valid: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PersistentValueSubmitRequest {
    pub value: String,
    pub duration: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PersistentValueSubmitResponse {
    pub id: Uuid,
    pub begin: NaiveDateTime,
    pub end: NaiveDateTime,
}
