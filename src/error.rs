use crate::duration::error::InvalidDurationError;
use crate::transient::error::{TransientValueDoesNotExistError, TransientValueExistsError};

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> ApiError {
        ApiError {
            status_code,
            message,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<TransientValueExistsError> for ApiError {
    fn from(from: TransientValueExistsError) -> Self {
        ApiError {
            status_code: 400,
            message: format!(
                "Attempting to insert the id {} when it already exists in the transient storage.",
                from.id
            ),
        }
    }
}

impl From<InvalidDurationError> for ApiError {
    fn from(from: InvalidDurationError) -> Self {
        ApiError {
            status_code: 400,
            message: format!(
                "Attempting to create an unsupported duration of {} seconds. This problem originates from the fact that we are unable to cast this `u64` to `i64`.",
                from.duration
            ),
        }
    }
}

impl From<TransientValueDoesNotExistError> for ApiError {
    fn from(from: TransientValueDoesNotExistError) -> Self {
        ApiError {
            status_code: 400,
            message: format!(
                "Attempting to know if the id {} is valid when it does not exist in the transient storage.",
                from.id
            ),
        }
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        match error {
            DieselError::DatabaseError(_, err) => ApiError::new(409, err.message().to_string()),
            DieselError::NotFound => ApiError::new(404, "Record not found".to_string()),
            err => ApiError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => {
                error!("{}", self.message);
                "Internal server error".to_string()
            }
        };

        HttpResponse::build(status_code).json(json!({ "message": message }))
    }
}
