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

pub struct TransientDictionary {
    pub map: HashMap<String, Duration>,
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
        if let Some(key) = self.map.get(&request.value) {
            let now = Utc::now().naive_utc();
            Ok(TransientValueValidityResponse {
                valid: key.valid(now),
            })
        } else {
            Err(ApiError::from(TransientValueDoesNotExistError::new(
                request.value,
            )))
        }
    }

    pub fn insert_value(
        &mut self,
        request: TransientValueSubmitRequest,
    ) -> Result<TransientValueSubmitResponse, ApiError> {
        if self.map.contains_key(&request.value) {
            Err(ApiError::from(TransientValueExistsError::new(
                request.value,
            )))
        } else {
            let duration = Duration::create_duration_seconds(request.duration)?;
            let result = self.map.entry(request.value).or_insert(duration).clone();
            Ok(TransientValueSubmitResponse {
                id: result.id,
                begin: result.begin,
                end: result.end,
            })
        }
    }
}
