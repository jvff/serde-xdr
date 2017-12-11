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

        /// Map types are not supported by XDR.
        MapIsNotSupported {
            description("XDR does not support a map type")
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

        /// Fatal error while serializing a sequence or a tuple.
        ///
        /// This is probably caused by ignoring a previous error.
        SerializeSequenceOrTupleFatalError(type_name: String) {
            description("fatal failure while serializing tuple or sequence")
            display("fatal failure while serializing {}", type_name)
        }

        /// Sequences with unknown lengths are not supported.
        SerializeSequenceWithUnknownLength {
            description("can't serialize sequence with unknown length")
        }

        /// Fatal error while serializing an object.
        ///
        /// This is probably caused by ignoring a previous error.
        SerializeStructFatalError(name: String) {
            description("fatal failure while serializing struct")
            display("fatal failure while serializing struct: {}", name)
        }

        /// Attempt to serialize a tuple that has too many elements.
        TupleHasTooManyElements(count: usize) {
            description("tuple has too many elements")
            display(
                "tuple has too many elements (maximum is {}): {}",
                u32::max_value(),
                count,
            )
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
