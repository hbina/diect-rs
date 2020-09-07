pub struct TransientValueExistsError {
    pub value: String,
}

impl TransientValueExistsError {
    pub fn new(value: String) -> TransientValueExistsError {
        TransientValueExistsError { value }
    }
}

pub struct TransientValueDoesNotExistError {
    pub value: String,
}

impl TransientValueDoesNotExistError {
    pub fn new(value: String) -> TransientValueDoesNotExistError {
        TransientValueDoesNotExistError { value }
    }
}
