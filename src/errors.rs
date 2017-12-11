use std::fmt::Display;
use std::io;

use serde::{ser, de};

use super::de::{CompatDeserializationError, DeserializationError};
use super::ser::SerializationError;

error_chain! {
    errors {
        /// Custom error type.
        Custom(message: String) {
            description("custom error")
            display("{}", message)
        }

        /// Wrapped deserialization error in the new format.
        DeserializationError(error: DeserializationError) {
            description("deserialization error")
            display("deserialization error: {}", error)
        }

        /// Wrapped serialization error in the new format.
        SerializationError(error: SerializationError) {
            description("serialization error")
            display("serialization error: {}", error)
        }

        /// Failure to serialize a value.
        SerializeFailure(what: String) {
            description("failed to serialize a value")
            display("failed to serialize {}", what)
        }
    }

    foreign_links {
        Io(io::Error);
    }
}

impl From<CompatDeserializationError> for ErrorKind {
    fn from(error: CompatDeserializationError) -> ErrorKind {
        ErrorKind::DeserializationError(error.into())
    }
}

impl From<CompatDeserializationError> for Error {
    fn from(error: CompatDeserializationError) -> Error {
        let error_kind: ErrorKind = error.into();

        error_kind.into()
    }
}

impl From<DeserializationError> for ErrorKind {
    fn from(error: DeserializationError) -> ErrorKind {
        ErrorKind::DeserializationError(error)
    }
}

impl From<DeserializationError> for Error {
    fn from(error: DeserializationError) -> Error {
        let error_kind: ErrorKind = error.into();

        error_kind.into()
    }
}

impl From<SerializationError> for ErrorKind {
    fn from(error: SerializationError) -> ErrorKind {
        ErrorKind::SerializationError(error)
    }
}

impl From<SerializationError> for Error {
    fn from(error: SerializationError) -> Error {
        let error_kind: ErrorKind = error.into();

        error_kind.into()
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(message: T) -> Self {
        ErrorKind::Custom(message.to_string()).into()
    }
}

impl de::Error for Error {
    fn custom<T: Display>(message: T) -> Self {
        ErrorKind::Custom(message.to_string()).into()
    }
}
