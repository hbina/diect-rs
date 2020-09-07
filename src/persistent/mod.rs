use crate::error::ApiError;
use actix_web::{get, post, web, HttpResponse};

#[get("/persistent/get")]
async fn get_persistent_value() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().finish())
}

#[post("/persistent/submit")]
async fn submit_persistent_value() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().finish())
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
