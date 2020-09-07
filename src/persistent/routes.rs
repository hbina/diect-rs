use crate::error::ApiError;
use crate::persistent::form::{PersistentValueSubmitRequest, PersistentValueValidityRequest};
use crate::persistent::persistent_value::PersistentStorageValueProxy;

use actix_web::{get, post, web, HttpResponse};

#[get("/persistent/get")]
pub async fn get_persistent_value(
    get_request: web::Query<PersistentValueValidityRequest>,
) -> Result<HttpResponse, ApiError> {
    let result = PersistentStorageValueProxy::find_if_value_valid(get_request.0)?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/persistent/submit")]
pub async fn submit_persistent_value(
    submit_request: web::Query<PersistentValueSubmitRequest>,
) -> Result<HttpResponse, ApiError> {
    let result = PersistentStorageValueProxy::submit_value(submit_request.0)?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/persistent/cleanup")]
pub async fn cleanup_persistent_storage() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().finish())
}
