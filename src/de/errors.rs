#[derive(Debug, Fail)]
pub enum DeserializationError {
    /// Failure while deserializing a value.
    #[fail(display = "failed to deserialize a value of type: {}", type_name)]
    Failure { type_name: String },

    /// Deserialization of an identifier (for meta-data) is not supported.
    #[fail(display = "deserialization of an identifier is not supported")]
    IdentifierNotSupported,

    /// Deserialized boolean value is invalid.
    #[fail(display = "deserialized an invalid bool: {}", raw_value)]
    InvalidBool { raw_value: u32 },

    /// Deserialized character value is invalid.
    #[fail(display = "deserialized an invalid char: 0x{:X}", raw_value)]
    InvalidChar { raw_value: u32 },

    /// Deserialized signed integer is invalid.
    #[fail(display = "deserialized invalid {}-bit signed integer: {}", bits,
           value)]
    InvalidInteger { bits: u8, value: i32 },

    /// Deserialized optional value is invalid.
    #[fail(display = "deserialized an invalid option")]
    InvalidOption,

    /// Deserialized unsigned integer is invalid.
    #[fail(display = "deserialized invalid {}-bit unsigned integer: {}", bits,
           value)]
    InvalidUnsignedInteger { bits: u8, value: u32 },

    /// Attempt to deserialize an unknown type.
    #[fail(display = "can't deserialize unknown type")]
    UnknownType,
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
