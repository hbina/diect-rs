use crate::duration::error::InvalidDurationError;
use crate::persistent::persistent_value::PersistentStorageValueProxy;

use chrono::Duration as ChronoDuration;
use chrono::{NaiveDateTime, Utc};
// use num_traits::cast::ToPrimitive;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Duration {
    pub begin: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl Duration {
    pub fn create_duration_seconds(s: u64) -> Result<Duration, InvalidDurationError> {
        if let Some(s) = s.to_i64() {
            let now = Utc::now().naive_utc();
            let end = now.add(ChronoDuration::seconds(s));
            Ok(Duration { begin: now, end })
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
            begin: from.date_begin,
            end: from.date_end,
        }
    }
}

pub mod error;
