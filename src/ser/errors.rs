#[derive(Debug, Fail)]
pub enum SerializationError {
    /// Custom error message.
    #[fail(display = "custom error message: {}", message)]
    Custom { message: String },

    /// Attempt to serialize a sequence that's too long.
    #[fail(display = "sequence is too long to be serialized: {}", length)]
    SequenceTooLong { length: usize },
}
