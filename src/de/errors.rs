use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::result;
use std::string::FromUtf8Error;

use failure::{Compat, Fail};
use serde::de;

/// Error during deserialization.
#[derive(Debug, Fail)]
pub enum DeserializationError {
    /// Custom error message.
    #[fail(display = "custom error message: {}", message)]
    Custom {
        /// The message of the custom error.
        message: String,
    },

    /// Failure while deserializing a value.
    #[fail(display = "failed to deserialize a value of type: {}", type_name)]
    Failure {
        /// The name of the type that was being deserialized.
        type_name: String,
        /// The error that ocurred during deserialization.
        #[cause]
        cause: Box<CompatDeserializationError>,
    },

    /// Deserialization of an identifier (for meta-data) is not supported.
    #[fail(display = "deserialization of an identifier is not supported")]
    IdentifierNotSupported,

    /// Deserialized boolean value is invalid.
    #[fail(display = "deserialized an invalid bool: {}", raw_value)]
    InvalidBool {
        /// The deserialized value that's not a zero or a one.
        raw_value: u32,
    },

    /// Deserialized character value is invalid.
    #[fail(display = "deserialized an invalid char: 0x{:X}", raw_value)]
    InvalidChar {
        /// The deserialized value that's larger than a byte.
        raw_value: u32,
    },

    /// Deserialized enum variant is invalid.
    #[fail(
        display = "deserialized an invalid enum variant: variant index is {}, valid variants are {:?}",
        variant, variants
    )]
    InvalidEnumVariant {
        /// The invalid variant index.
        variant: u32,
        /// The valid variants.
        variants: &'static [&'static str],
    },

    /// Deserialized signed integer is invalid.
    #[fail(
        display = "deserialized invalid {}-bit signed integer: {}",
        bits, value
    )]
    InvalidInteger {
        /// The maximum number of bits expected.
        bits: u8,
        /// The deserialized value that has more bits than expected.
        value: i32,
    },

    /// Deserialized optional value is invalid.
    #[fail(display = "deserialized an invalid option")]
    InvalidOption,

    /// Deserialized an invalid UTF-8 string.
    #[fail(display = "deserialized an invalid UTF-8 string")]
    InvalidString {
        /// The error that ocurred while converting the deserialized bytes into
        /// a string.
        cause: FromUtf8Error,
    },

    /// Deserialized unsigned integer is invalid.
    #[fail(
        display = "deserialized invalid {}-bit unsigned integer: {}",
        bits, value
    )]
    InvalidUnsignedInteger {
        /// The maximum number of bits expected.
        bits: u8,
        /// The deserialized value that has more bits than expected.
        value: u32,
    },

    /// IO error while deserializing a value.
    #[fail(
        display = "IO error while deserializing a value of type {}: {}",
        type_name, cause
    )]
    IoError {
        /// The name of the type that was being deserialized.
        type_name: String,
        /// The error that ocurred during deserialization.
        #[cause]
        cause: io::Error,
    },

    /// Map types are not supported by XDR.
    #[fail(display = "XDR does not support a map type")]
    MapIsNotSupported,

    /// Attempt to deserialize a tuple that has too many elements.
    #[fail(display = "tuple has too many elements: {}", length)]
    TupleHasTooManyElements {
        /// Number of elements that were expected, but can't be represented.
        length: usize,
    },

    /// Attempt to deserialize an unknown type.
    #[fail(display = "can't deserialize unknown type")]
    UnknownType,
}

impl DeserializationError {
    /// Create a deserialization failure error.
    pub fn failure<S, E>(type_name: S, cause: E) -> Self
    where
        S: ToString,
        E: Into<CompatDeserializationError>,
    {
        DeserializationError::Failure {
            type_name: type_name.to_string(),
            cause: Box::new(cause.into()),
        }
    }

    /// Create a deserialization IO error.
    pub fn io_error<S>(type_name: S, cause: io::Error) -> Self
    where
        S: ToString,
    {
        DeserializationError::IoError {
            type_name: type_name.to_string(),
            cause,
        }
    }
}

impl From<CompatDeserializationError> for DeserializationError {
    fn from(wrapped_error: CompatDeserializationError) -> Self {
        match wrapped_error {
            CompatDeserializationError(error) => error.into_inner(),
        }
    }
}

/// An `Error`-compatible wrapper for `DeserializationError`.
///
/// Contains helper methods to convert to and from the wrapped type.
#[derive(Debug)]
pub struct CompatDeserializationError(Compat<DeserializationError>);

impl From<DeserializationError> for CompatDeserializationError {
    fn from(error: DeserializationError) -> Self {
        CompatDeserializationError(error.compat())
    }
}

impl Display for CompatDeserializationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl error::Error for CompatDeserializationError {
    fn description(&self) -> &str {
        self.0.description()
    }
}

impl de::Error for CompatDeserializationError {
    fn custom<T: Display>(message: T) -> Self {
        let error = DeserializationError::Custom {
            message: message.to_string(),
        };

        CompatDeserializationError(error.compat())
    }
}

pub type Result<T> = result::Result<T, CompatDeserializationError>;
