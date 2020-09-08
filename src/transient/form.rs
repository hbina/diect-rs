use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransientValueValidityRequest {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransientValueValidityResponse {
    pub valid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransientValueSubmitRequest {
    pub duration: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransientValueSubmitResponse {
    pub id: Uuid,
    pub begin: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransientCleanupResponse {
    pub amount_cleaned_up: usize,
}
