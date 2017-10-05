use std::fmt::Display;
use std::io;

use serde::{ser, de};

error_chain! {
    errors {
        /// Custom error type.
        Custom(message: String) {
            description("custom error")
            display("{}", message)
        }

        /// Failure while deserializing a value.
        DeserializeFailure(type_name: String) {
            description("failed to deserialize a value")
            display("failed to deserialize a value of type {}", type_name)
        }

        /// Attempt to deserialize an unknown type.
        DeserializeUnknownType {
            description("can't deserialize unknown type")
        }

        /// Deserialization of an identifier (for meta-data) is not supported.
        IdentifierNotSupported {
            description("deserialization of an identifier is not supported")
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

        /// Attempt to serialize a sequence that's too long.
        SequenceTooLong(length: usize) {
            description("sequence is too long to be serialized")
            display("sequence is too long to be serialized: {}", length)
        }

        /// Failure to serialize a boolean value.
        SerializeBool(value: bool) {
            description("failed to serialize bool")
            display("failed to serialize bool: {}", value)
        }

        /// Failure to serialize a character value.
        SerializeChar(value: char) {
            description("failed to serialize char")
            display("failed to serialize char: '{}'", value.escape_default())
        }

        /// Failure to serialize a double precision floating point value.
        SerializeDouble(value: f64) {
            description("failed to serialize double")
            display("failed to serialize double: {}", value)
        }

        /// Failure to serialize an enumeration variant.
        SerializeEnum(name: String, variant: String) {
            description("failed to serialize enum variant")
            display("failed to serialize enum variant: {}::{}", name, variant)
        }

        /// Failure to serialize a single precision floating point value.
        SerializeFloat(value: f32) {
            description("failed to serialize float")
            display("failed to serialize float: {}", value)
        }

        /// Failure to serialize a 64-bit signed integer.
        SerializeHyperInteger(value: i64) {
            description("failed to serialize hyper integer")
            display("failed to serialize hyper integer: {}", value)
        }

        /// Failure to serialize a signed integer.
        SerializeInteger(value: i32) {
            description("failed to serialize integer")
            display("failed to serialize integer: {}", value)
        }

        /// Failure to serialize a marker that specifies that an optional value
        /// is not present.
        SerializeNone {
            description("failed to serialize 'none' optional variant")
        }

        /// Failure to serialize variable length opaque data.
        SerializeOpaque(size: usize) {
            description("failed to serialize opaque")
            display("failed to serialize opaque data of {} bytes", size)
        }

        /// Failure to serialize some optional data that is present.
        SerializeSome {
            description("failed to serialize some optional data")
        }

        /// Failure to serialize a string.
        SerializeString(string: String) {
            description("failed to serialize string")
            display("failed to serialize string: {}", string)
        }

        /// Failure to serialize an element of a sequence or a tuple.
        SerializeSequenceOrTupleElement(type_name: String, index: usize) {
            description("failed to serialize sequence or tuple element")
            display("failed to serialize {} element {}", type_name, index)
        }

        /// Fatal error while serializing a sequence or a tuple.
        ///
        /// This is probably caused by ignoring a previous error.
        SerializeSequenceOrTupleFatalError(type_name: String) {
            description("fatal failure while serializing tuple or sequence")
            display("fatal failure while serializing {}", type_name)
        }

        /// Failure to serialize sequence length value.
        SerializeSequenceLength(length: usize) {
            description("failed to serialize sequence length")
            display("failed to serialize sequence length: {}", length)
        }

        /// Sequences with unknown lengths are not supported.
        SerializeSequenceWithUnknownLength {
            description("can't serialize sequence with unknown length")
        }

        /// Sequences with unknown lengths are not supported.
        SerializeStruct(name: String) {
            description("failed to serialize struct")
            display("failed to serialize struct: {}", name)
        }

        /// Fatal error while serializing an object.
        ///
        /// This is probably caused by ignoring a previous error.
        SerializeStructFatalError(name: String) {
            description("fatal failure while serializing struct")
            display("fatal failure while serializing struct: {}", name)
        }

        /// Failure to serialize an object field value.
        SerializeStructField(struct_name: String, field_name: String) {
            description("failed to serialize struct field")
            display(
                "failed to serialize struct field: {}::{}",
                struct_name,
                field_name,
            )
        }

        /// Failure to serialize a variant of a union.
        SerializeUnionVariant(name: String, variant: String) {
            description("failed to serialize union variant")
            display("failed to serialize union variant: {}::{}", name, variant)
        }

        /// Failure to serialize a 64-bit unsigned integer.
        SerializeUnsignedHyperInteger(value: u64) {
            description("failed to serialize unsigned hyper integer")
            display("failed to serialize unsigned hyper integer: {}", value)
        }

        /// Failure to serialize an unsigned integer.
        SerializeUnsignedInteger(value: u32) {
            description("failed to serialize unsigned integer")
            display("failed to serialize unsigned integer: {}", value)
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
