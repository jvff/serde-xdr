#[derive(Debug, Fail)]
pub enum SerializationError {
    /// Custom error message.
    #[fail(display = "custom error message: {}", message)]
    Custom { message: String },

    /// Attempt to serialize opaque data with too many bytes.
    #[fail(display = "opaque data is too long: {} bytes", length)]
    OpaqueDataIsTooLong { length: usize },

    /// Attempt to serialize a sequence that's too long.
    #[fail(display = "sequence is too long to be serialized: {}", length)]
    SequenceTooLong { length: usize },
}
