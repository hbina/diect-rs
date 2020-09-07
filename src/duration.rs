use crate::error::ApiError;

use crate::persistent::PersistentStorageValueProxy;
use actix_web::{get, post, web, HttpResponse};
use chrono::Duration as ChronoDuration;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Add;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Duration {
    pub id: Uuid,
    pub begin: NaiveDateTime,
    pub end: NaiveDateTime,
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
