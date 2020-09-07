use crate::error::ApiError;
use actix_web::{get, post, web, HttpResponse};
use chrono::Duration as ChronoDuration;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Add;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Duration {
    id: Uuid,
    begin: NaiveDateTime,
    end: NaiveDateTime,
}

impl Duration {
    pub fn create_duration_seconds(s: u64) -> Duration {
        // TODO :: This should be checked. Why the fuck are you attempting to make something last
        // longer than the universe?
        let s = s as i64;
        let now = Utc::now().naive_utc();
        let end = now.add(ChronoDuration::seconds(s));
        Duration {
            id: Uuid::new_v4(),
            begin: now,
            end,
        }
    }

    // TODO :: Also implement the other stuffs...

    fn valid(&self, time: NaiveDateTime) -> bool {
        time > self.begin && time < self.end
    }
}

pub struct TransientDictionary {
    map: HashMap<String, Duration>,
}

pub struct ValueExistsError {
    pub value: String,
}

impl ValueExistsError {
    pub fn new(value: String) -> ValueExistsError {
        ValueExistsError { value }
    }
}

pub struct ValueDoesNotExistError {
    pub value: String,
}

impl ValueDoesNotExistError {
    pub fn new(value: String) -> ValueDoesNotExistError {
        ValueDoesNotExistError { value }
    }
}

impl TransientDictionary {
    pub fn create_storage() -> web::Data<Mutex<Self>> {
        web::Data::new(Mutex::new(TransientDictionary {
            map: std::collections::HashMap::new(),
        }))
    }

    fn find_if_value_valid(
        &self,
        request: TransientValueValidityRequest,
    ) -> Result<TransientValueValidityResponse, ApiError> {
        if let Some(key) = self.map.get(&request.value) {
            let now = Utc::now().naive_utc();
            Ok(TransientValueValidityResponse {
                valid: key.valid(now),
            })
        } else {
            Err(ApiError::from(ValueDoesNotExistError::new(request.value)))
        }
    }

    fn insert_value(
        &mut self,
        request: TransientValueSubmitRequest,
    ) -> Result<TransientValueSubmitResponse, ApiError> {
        if self.map.contains_key(&request.value) {
            Err(ApiError::from(ValueExistsError::new(request.value)))
        } else {
            let duration = Duration::create_duration_seconds(request.duration);
            let result = self.map.entry(request.value).or_insert(duration).clone();
            Ok(TransientValueSubmitResponse {
                id: result.id,
                begin: result.begin,
                end: result.end,
            })
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TransientValueValidityRequest {
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TransientValueValidityResponse {
    valid: bool,
}

#[get("/transient/get")]
async fn get_transient_value(
    get_request: web::Query<TransientValueValidityRequest>,
    storage: web::Data<Mutex<TransientDictionary>>,
) -> Result<HttpResponse, ApiError> {
    let storage = storage.lock().unwrap();
    let request = get_request.0;
    Ok(HttpResponse::Ok().json(storage.find_if_value_valid(request)?))
}

#[derive(Serialize, Deserialize, Debug)]
struct TransientValueSubmitRequest {
    value: String,
    duration: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct TransientValueSubmitResponse {
    id: Uuid,
    begin: NaiveDateTime,
    end: NaiveDateTime,
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

#[derive(Serialize, Deserialize, Debug)]
struct TransientCleanupResponse {
    amount_cleaned_up: usize,
}

#[post("/transient/cleanup")]
async fn cleanup_transient_storage(
    storage: web::Data<Mutex<TransientDictionary>>,
) -> Result<HttpResponse, ApiError> {
    let mut storage = storage.lock().unwrap();
    let now = Utc::now().naive_utc();
    let len_before = storage.map.len();
    storage.map.retain(|_, y| y.begin > now);
    let len_after = storage.map.len();
    Ok(HttpResponse::Ok().json(TransientCleanupResponse {
        amount_cleaned_up: len_before - len_after,
    }))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_transient_value);
    cfg.service(submit_transient_value);
    cfg.service(cleanup_transient_storage);
}
