use std::fmt::Display;
use std::io;

use serde::{ser, de};

use super::de::DeserializationError;

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

        /// Deserialized boolean value is invalid.
        InvalidBool(raw_value: u32) {
            description("deserialized an invalid bool")
            display("deserialized an invalid bool: {}", raw_value)
        }

        /// Deserialized character value is invalid.
        InvalidChar(raw_value: u32) {
            description("deserialized an invalid char")
            display("deserialized an invalid char: 0x{:X}", raw_value)
        }

        /// Deserialized optional value is invalid.
        InvalidOption {
            description("deserialized an invalid option")
        }

        /// Deserialized signed integer is invalid.
        InvalidInteger(bits: u8, value: i32) {
            description("deserialized invalid signed integer")
            display(
                "deserialized invalid {}-bit signed integer: {}",
                bits,
                value,
            )
        }

        /// Deserialized unsigned integer is invalid.
        InvalidUnsignedInteger(bits: u8, value: u32) {
            description("deserialized invalid unsigned integer")
            display(
                "deserialized invalid {}-bit unsigned integer: {}",
                bits,
                value,
            )
        }

        /// Map types are not supported by XDR.
        MapIsNotSupported {
            description("XDR does not support a map type")
        }

        /// Attempt to serialize opaque data with too many bytes
        OpaqueDataIsTooLong(length: usize) {
            description("opaque data is too long")
            display(
                concat!(
                    "opaque data is too long (maximum length is {} bytes): {}",
                    " bytes",
                ),
                u32::max_value(),
                length,
            )
        }

        /// Attempt to serialize a sequence that's too long.
        SequenceTooLong(length: usize) {
            description("sequence is too long to be serialized")
            display("sequence is too long to be serialized: {}", length)
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

        /// Only ASCII strings can be serialized
        StringIsNotAscii(string: String) {
            description("string is not ASCII encoded")
            display("string is not ASCII encoded: {}", string)
        }

        /// Attempt to serialize a string that's too long.
        StringIsTooLong(string: String) {
            description("string is too long")
            display(
                "string is too long (maximum length is {} bytes): {}",
                u32::max_value(),
                string,
            )
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
