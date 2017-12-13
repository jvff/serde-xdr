use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::result;

use failure::{Compat, Fail};
use serde::ser;

#[derive(Debug, Fail)]
pub enum SerializationError {
    /// Custom error message.
    #[fail(display = "custom error message: {}", message)]
    Custom { message: String },

    /// Failure to serialize a value.
    #[fail(display = "failed to serialize {}", what)]
    Failure {
        what: String,
        #[cause]
        cause: Box<CompatSerializationError>,
    },

    /// IO error while serializing a value.
    #[fail(display = "IO error while serializing {}: {}", what, cause)]
    IoError {
        what: String,
        #[cause]
        cause: io::Error,
    },

    /// Map types are not supported by XDR.
    #[fail(display = "XDR does not support a map type")]
    MapIsNotSupported,

    /// Attempt to serialize opaque data with too many bytes.
    #[fail(display = "opaque data is too long: {} bytes", length)]
    OpaqueDataIsTooLong { length: usize },

    /// Fatal error while serializing a sequence or a tuple.
    ///
    /// This is probably caused by ignoring a previous error.
    #[fail(display = "fatal failure while serializing {}", type_name)]
    SequenceOrTupleFatalError { type_name: String },

    /// Fatal error while serializing an object.
    ///
    /// This is probably caused by ignoring a previous error.
    #[fail(display = "fatal failure while serializing struct: {}", name)]
    StructFatalError { name: String },

    /// Attempt to serialize a sequence that's too long.
    #[fail(display = "sequence is too long to be serialized: {}", length)]
    SequenceTooLong { length: usize },

    /// Sequences with unknown lengths are not supported.
    #[fail(display = "can't serialize sequence with unknown length")]
    SequenceWithUnknownLength,

    /// Only ASCII strings can be serialized.
    #[fail(display = "string is not ASCII encoded: {}", string)]
    StringIsNotAscii { string: String },

    /// Attempt to serialize a string that's too long.
    #[fail(display = "string is too long: {}", string)]
    StringIsTooLong { string: String },
}

#[derive(Debug)]
pub struct CompatSerializationError(Compat<SerializationError>);

impl From<SerializationError> for CompatSerializationError {
    fn from(error: SerializationError) -> Self {
        CompatSerializationError(error.compat())
    }
}

impl Display for CompatSerializationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl error::Error for CompatSerializationError {
    fn description(&self) -> &str {
        self.0.description()
    }
}

impl ser::Error for CompatSerializationError {
    fn custom<T: Display>(message: T) -> Self {
        let error = SerializationError::Custom {
            message: message.to_string(),
        };

        CompatSerializationError(error.compat())
    }
}

impl From<CompatSerializationError> for SerializationError {
    fn from(wrapped_error: CompatSerializationError) -> Self {
        match wrapped_error {
            CompatSerializationError(error) => error.into_inner(),
        }
    }
}

pub type Result<T> = result::Result<T, CompatSerializationError>;
