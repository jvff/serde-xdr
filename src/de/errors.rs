#[derive(Debug, Fail)]
pub enum DeserializationError {
    /// Failure while deserializing a value.
    #[fail(display = "failed to deserialize a value of type: {}", type_name)]
    Failure { type_name: String },
}
