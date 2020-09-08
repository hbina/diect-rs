use crate::duration::Duration;
use crate::error::ApiError;
use crate::transient::error::{TransientValueDoesNotExistError, TransientValueExistsError};
use crate::transient::form::{
    TransientValueSubmitRequest, TransientValueSubmitResponse, TransientValueValidityRequest,
    TransientValueValidityResponse,
};

use actix_web::web;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub struct TransientDictionary {
    pub map: HashMap<Uuid, Duration>,
}

impl TransientDictionary {
    pub fn create_storage() -> web::Data<Mutex<Self>> {
        web::Data::new(Mutex::new(TransientDictionary {
            map: std::collections::HashMap::new(),
        }))
    }

    pub fn find_if_value_valid(
        &self,
        request: TransientValueValidityRequest,
    ) -> Result<TransientValueValidityResponse, ApiError> {
        if let Some(key) = self.map.get(&request.id) {
            let now = Utc::now().naive_utc();
            Ok(TransientValueValidityResponse {
                valid: key.valid(now),
            })
        } else {
            Err(ApiError::from(TransientValueDoesNotExistError {
                id: request.id,
            }))
        }
    }

    pub fn insert_value(
        &mut self,
        request: TransientValueSubmitRequest,
    ) -> Result<TransientValueSubmitResponse, ApiError> {
        let id = Uuid::new_v4();
        if self.map.contains_key(&id) {
            Err(ApiError::from(TransientValueExistsError { id }))
        } else {
            let duration = Duration::create_duration_seconds(request.duration)?;
            let result = self.map.entry(id).or_insert(duration);
            Ok(TransientValueSubmitResponse {
                id,
                begin: result.begin,
                end: result.end,
            })
        }
    }
}
