#[derive(Debug, Fail)]
pub enum SerializationError {
    /// Custom error message.
    #[fail(display = "custom error message: {}", message)]
    Custom { message: String },
}
