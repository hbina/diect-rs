use uuid::Uuid;

pub struct TransientValueExistsError {
    pub id: Uuid,
}

pub struct TransientValueDoesNotExistError {
    pub id: Uuid,
}
