table! {
    persistent_storage (id) {
        id -> Uuid,
        value_text -> Text,
        date_begin -> Timestamp,
        date_end -> Timestamp,
    }
}
