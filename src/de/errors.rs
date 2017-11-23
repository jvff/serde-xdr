#[derive(Debug, Fail)]
pub enum DeserializationError {
    /// Failure while deserializing a value.
    #[fail(display = "failed to deserialize a value of type: {}", type_name)]
    Failure { type_name: String },
}

impl DeserializationError {
    pub fn failure<S>(type_name: S) -> Self
    where
        S: ToString,
    {
        DeserializationError::Failure {
            type_name: type_name.to_string(),
        }
    }
}
