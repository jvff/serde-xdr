use {
    failure::{Compat, Fail},
    serde::ser,
    std::{
        error,
        fmt::{self, Display, Formatter},
        io, result,
    },
};

/// Error during serialization.
#[derive(Debug, Fail)]
pub enum SerializationError {
    /// Custom error message.
    #[fail(display = "custom error message: {}", message)]
    Custom {
        /// The message of the custom error.
        message: String,
    },

    /// Failure to serialize a value.
    #[fail(display = "failed to serialize {}", what)]
    Failure {
        /// A description of what was being serialized.
        what: String,
        /// The error that ocurred during serialization.
        #[cause]
        cause: Box<CompatSerializationError>,
    },

    /// IO error while serializing a value.
    #[fail(display = "IO error while serializing {}: {}", what, cause)]
    IoError {
        /// A description of what was being serialized.
        what: String,
        /// The error that ocurred during serialization.
        #[cause]
        cause: io::Error,
    },

    /// Map types are not supported by XDR.
    #[fail(display = "XDR does not support a map type")]
    MapIsNotSupported,

    /// Attempt to serialize opaque data with too many bytes.
    #[fail(display = "opaque data is too long: {} bytes", length)]
    OpaqueDataIsTooLong {
        /// The length of the data, which is larger than what can be
        /// represented.
        length: usize,
    },

    /// Fatal error while serializing a sequence or a tuple.
    ///
    /// This is probably caused by ignoring a previous error.
    #[fail(display = "fatal failure while serializing {}", type_name)]
    SequenceOrTupleFatalError {
        /// The name of the type being serialized.
        type_name: String,
    },

    /// Fatal error while serializing an object.
    ///
    /// This is probably caused by ignoring a previous error.
    #[fail(display = "fatal failure while serializing struct: {}", name)]
    StructFatalError {
        /// The name of the type being serialized.
        name: String,
    },

    /// Attempt to serialize a sequence that's too long.
    #[fail(display = "sequence is too long to be serialized: {}", length)]
    SequenceTooLong {
        /// The length of the sequence, which is larger than what can be
        /// represented.
        length: usize,
    },

    /// Sequences with unknown lengths are not supported.
    #[fail(display = "can't serialize sequence with unknown length")]
    SequenceWithUnknownLength,

    /// Only ASCII strings can be serialized.
    #[fail(display = "string is not ASCII encoded: {}", string)]
    StringIsNotAscii {
        /// The string that can't be represented as an ASCII string.
        string: String,
    },

    /// Attempt to serialize a string that's too long.
    #[fail(display = "string is too long: {}", string)]
    StringIsTooLong {
        /// The length of the string, which is larger than what can be
        /// represented.
        string: String,
    },
}

/// An `Error`-compatible wrapper for `SerializationError`.
///
/// Contains helper methods to convert to and from the wrapped type.
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
