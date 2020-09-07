use crate::duration::error::InvalidDurationError;
use crate::persistent::persistent_value::PersistentStorageValueProxy;

use chrono::Duration as ChronoDuration;
use chrono::{NaiveDateTime, Utc};
// use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Duration {
    pub id: Uuid,
    pub begin: NaiveDateTime,
    pub end: NaiveDateTime,
}

// FIXME: Replace with `num_traits` once you have internet.
fn convert_to_i64(s: u64) -> Option<i64> {
    if s < 100000 {
        Some(s as i64)
    } else {
        None
    }
}

impl Duration {
    pub fn create_duration_seconds(s: u64) -> Result<Duration, InvalidDurationError> {
        if let Some(s) = convert_to_i64(s) {
            let now = Utc::now().naive_utc();
            let end = now.add(ChronoDuration::seconds(s));
            Ok(Duration {
                id: Uuid::new_v4(),
                begin: now,
                end,
            })
        } else {
            Err(InvalidDurationError { duration: s })
        }
    }

    pub fn valid(&self, time: NaiveDateTime) -> bool {
        time > self.begin && time < self.end
    }

    pub fn should_be_cleaned_up(&self, time: NaiveDateTime) -> bool {
        time > self.begin
    }
}

impl From<PersistentStorageValueProxy> for Duration {
    fn from(from: PersistentStorageValueProxy) -> Duration {
        Duration {
            id: from.id,
            begin: from.date_begin,
            end: from.date_end,
        }
    }
}

pub mod error;
