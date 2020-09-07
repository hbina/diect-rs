use crate::db;
use crate::duration::Duration;
use crate::error::ApiError;
use crate::schema::persistent_storage;
use crate::schema::persistent_storage::value_text;

use actix_web::{get, post, web, HttpResponse};
use chrono::Duration as ChronoDuration;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use uuid::Uuid;

#[derive(Serialize, Debug, Queryable, Insertable, Deserialize)]
#[table_name = "persistent_storage"]
pub struct PersistentStorageValueProxy {
    pub id: Uuid,
    pub value_text: String,
    pub date_begin: NaiveDateTime,
    pub date_end: NaiveDateTime,
}

impl From<PersistentValueSubmitRequest> for PersistentStorageValueProxy {
    fn from(from: PersistentValueSubmitRequest) -> PersistentStorageValueProxy {
        let now = Utc::now().naive_utc();
        PersistentStorageValueProxy {
            id: Uuid::new_v4(),
            value_text: from.value,
            date_begin: now,
            date_end: now.add(ChronoDuration::seconds(from.duration as i64)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PersistentStorageValue {
    pub id: Uuid,
    pub value: String,
    pub begin: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl From<PersistentStorageValueProxy> for PersistentStorageValue {
    fn from(from: PersistentStorageValueProxy) -> PersistentStorageValue {
        PersistentStorageValue {
            id: from.id,
            value: from.value_text,
            begin: from.date_begin,
            end: from.date_end,
        }
    }
}

impl PersistentStorageValueProxy {
    pub fn find_if_value_valid(
        validity_request: PersistentValueValidityRequest,
    ) -> Result<PersistentValueValidityResponse, ApiError> {
        let conn = db::connection()?;
        let duration = Duration::from(
            persistent_storage::table
                .filter(value_text.eq(validity_request.value))
                .first::<PersistentStorageValueProxy>(&conn)?,
        );
        let now = Utc::now().naive_utc();
        Ok(PersistentValueValidityResponse {
            valid: duration.valid(now),
        })
    }

    pub fn submit_value(
        submit_request: PersistentValueSubmitRequest,
    ) -> Result<PersistentValueSubmitResponse, ApiError> {
        let conn = db::connection()?;
        let submission = PersistentStorageValueProxy::from(submit_request);
        let result = diesel::insert_into(persistent_storage::table)
            .values((
                persistent_storage::id.eq(submission.id),
                persistent_storage::value_text.eq(submission.value_text),
                persistent_storage::date_begin.eq(submission.date_begin),
                persistent_storage::date_end.eq(submission.date_end),
            ))
            .get_result::<PersistentStorageValueProxy>(&conn)?;
        Ok(PersistentValueSubmitResponse {
            id: result.id,
            begin: result.date_begin,
            end: result.date_end,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct PersistentValueValidityRequest {
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct PersistentValueValidityResponse {
    valid: bool,
}

#[get("/persistent/get")]
async fn get_persistent_value(
    get_request: web::Query<PersistentValueValidityRequest>,
) -> Result<HttpResponse, ApiError> {
    let result = PersistentStorageValueProxy::find_if_value_valid(get_request.0)?;
    Ok(HttpResponse::Ok().json(result))
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

#[post("/persistent/submit")]
async fn submit_persistent_value(
    submit_request: web::Query<PersistentValueSubmitRequest>,
) -> Result<HttpResponse, ApiError> {
    let result = PersistentStorageValueProxy::submit_value(submit_request.0)?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/persistent/cleanup")]
async fn cleanup_persistent_storage() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().finish())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_persistent_value);
    cfg.service(submit_persistent_value);
    cfg.service(cleanup_persistent_storage);
}
