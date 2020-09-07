use crate::duration::Duration;
use crate::error::ApiError;

use crate::transient::form::TransientValueSubmitRequest;
use actix_web::{get, post, web, HttpResponse};

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::sync::Mutex;
use uuid::Uuid;

pub(crate) mod error;
mod form;
mod routes;
pub(crate) mod transient_dictionary;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_transient_value);
    cfg.service(submit_transient_value);
    cfg.service(cleanup_transient_storage);
}
