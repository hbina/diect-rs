use crate::duration::Duration;
use crate::error::ApiError;

use crate::transient::form::{
    TransientCleanupResponse, TransientValueSubmitRequest, TransientValueValidityRequest,
};
use crate::transient::transient_dictionary::TransientDictionary;
use actix_web::{get, post, web, HttpResponse};
use chrono::Duration as ChronoDuration;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Add;
use std::sync::Mutex;
use uuid::Uuid;

#[get("/transient/get")]
async fn get_transient_value(
    get_request: web::Query<TransientValueValidityRequest>,
    storage: web::Data<Mutex<TransientDictionary>>,
) -> Result<HttpResponse, ApiError> {
    let storage = storage.lock().unwrap();
    let request = get_request.0;
    Ok(HttpResponse::Ok().json(storage.find_if_value_valid(request)?))
}

#[post("/transient/submit")]
async fn submit_transient_value(
    submit_request: web::Json<TransientValueSubmitRequest>,
    storage: web::Data<Mutex<TransientDictionary>>,
) -> Result<HttpResponse, ApiError> {
    let mut storage = storage.lock().unwrap();
    let request = submit_request.0;
    Ok(HttpResponse::Ok().json(storage.insert_value(request)?))
}

#[post("/transient/cleanup")]
async fn cleanup_transient_storage(
    storage: web::Data<Mutex<TransientDictionary>>,
) -> Result<HttpResponse, ApiError> {
    let mut storage = storage.lock().unwrap();
    let now = Utc::now().naive_utc();
    let len_before = storage.map.len();
    storage.map.retain(|_, y| y.should_be_cleaned_up(now));
    let len_after = storage.map.len();
    Ok(HttpResponse::Ok().json(TransientCleanupResponse {
        amount_cleaned_up: len_before - len_after,
    }))
}