use std::fmt::Display;
use std::io;

use serde::{ser, de};

error_chain! {
    errors {
        Custom(message: String) {
            description("custom error")
            display("{}", message)
        }

        DeserializeBool {
            description("failed to deserialize bool")
        }

        DeserializeChar {
            description("failed to deserialize char")
        }

        DeserializeDouble {
            description("failed to deserialize double")
        }

        DeserializeEnum(name: String) {
            description("failed to deserialize enum")
            display("failed to deserialize enum: {}", name)
        }

        DeserializeEnumVariant(name: String, variant: String) {
            description("failed to deserialize enum variant")
            display("failed to deserialize enum variant: {}::{}", name, variant)
        }

        DeserializeFloat {
            description("failed to deserialize float")
        }

        DeserializeInteger(bits: u8) {
            description("failed to deserialize signed integer")
            display("failed to deserialize {}-bit signed integer", bits)
        }

        DeserializeOpaque {
            description("failed to deserialize opaque data")
        }

        DeserializeOption {
            description("failed to deserialize option")
        }

        DeserializeSequence {
            description("failed to deserialize sequence")
        }

        DeserializeSequenceOrTupleElement(type_name: String, index: u32) {
            description("failed to deserialize sequence or tuple element")
            display(
                "failed to deserialize {} element at position {}",
                type_name,
                index,
            )
        }

        DeserializeString {
            description("failed to deserialize string")
        }

        DeserializeStruct(name: String) {
            description("failed to deserialize struct")
            display("failed to deserialize struct: {}", name)
        }

        DeserializeStructField(struct_name: String, field_name: String) {
            description("failed to deserialize struct field")
            display(
                "failed to deserialize struct field: {}::{}",
                struct_name,
                field_name,
            )
        }

        DeserializeUnknownType {
            description("can't deserialize unknown type")
        }

        DeserializeUnsignedInteger(bits: u8) {
            description("failed to deserialize unsigned integer")
            display("failed to deserialize {}-bit unsigned integer", bits)
        }

        IdentifierNotSupported {
            description("deserialization of an identifier is not supported")
        }

        InvalidBool(raw_value: u32) {
            description("deserialized an invalid bool")
            display("deserialized an invalid bool: {}", raw_value)
        }

        InvalidChar(raw_value: u32) {
            description("deserialized an invalid char")
            display("deserialized an invalid char: 0x{:X}", raw_value)
        }

        InvalidOption {
            description("deserialized an invalid option")
        }

        InvalidInteger(bits: u8, value: i32) {
            description("deserialized invalid signed integer")
            display(
                "deserialized invalid {}-bit signed integer: {}",
                bits,
                value,
            )
        }

        InvalidUnsignedInteger(bits: u8, value: u32) {
            description("deserialized invalid unsigned integer")
            display(
                "deserialized invalid {}-bit unsigned integer: {}",
                bits,
                value,
            )
        }

        MapIsNotSupported {
            description("XDR does not support a map type")
        }

        SequenceTooLong(length: usize) {
            description("sequence is too long to be serialized")
            display("sequence is too long to be serialized: {}", length)
        }

        SerializeBool(value: bool) {
            description("failed to serialize bool")
            display("failed to serialize bool: {}", value)
        }

        SerializeChar(value: char) {
            description("failed to serialize char")
            display("failed to serialize char: '{}'", value.escape_default())
        }

        SerializeDouble(value: f64) {
            description("failed to serialize double")
            display("failed to serialize double: {}", value)
        }

        SerializeEnum(name: String, variant: String) {
            description("failed to serialize enum variant")
            display("failed to serialize enum variant: {}::{}", name, variant)
        }

        SerializeFloat(value: f32) {
            description("failed to serialize float")
            display("failed to serialize float: {}", value)
        }

        SerializeHyperInteger(value: i64) {
            description("failed to serialize hyper integer")
            display("failed to serialize hyper integer: {}", value)
        }

        SerializeInteger(value: i32) {
            description("failed to serialize integer")
            display("failed to serialize integer: {}", value)
        }

        SerializeNone {
            description("failed to serialize 'none' optional variant")
        }

        SerializeOpaque(size: usize) {
            description("failed to serialize opaque")
            display("failed to serialize opaque data of {} bytes", size)
        }

        SerializeSome {
            description("failed to serialize some optional data")
        }

        SerializeString(string: String) {
            description("failed to serialize string")
            display("failed to serialize string: {}", string)
        }

        SerializeSequenceOrTupleElement(type_name: String, index: usize) {
            description("failed to serialize sequence or tuple element")
            display("failed to serialize {} element {}", type_name, index)
        }

        SerializeSequenceOrTupleFatalError(type_name: String) {
            description("fatal failure while serializing tuple or sequence")
            display("fatal failure while serializing {}", type_name)
        }

        SerializeSequenceLength(length: usize) {
            description("failed to serialize sequence length")
            display("failed to serialize sequence length: {}", length)
        }

        SerializeSequenceWithUnknownLength {
            description("can't serialize sequence with unknown length")
        }

        SerializeStruct(name: String) {
            description("failed to serialize struct")
            display("failed to serialize struct: {}", name)
        }

        SerializeStructFatalError(name: String) {
            description("fatal failure while serializing struct")
            display("fatal failure while serializing struct: {}", name)
        }

        SerializeStructField(struct_name: String, field_name: String) {
            description("failed to serialize struct field")
            display(
                "failed to serialize struct field: {}::{}",
                struct_name,
                field_name,
            )
        }

        SerializeUnionVariant(name: String, variant: String) {
            description("failed to serialize union variant")
            display("failed to serialize union variant: {}::{}", name, variant)
        }

        SerializeUnsignedHyperInteger(value: u64) {
            description("failed to serialize unsigned hyper integer")
            display("failed to serialize unsigned hyper integer: {}", value)
        }

        SerializeUnsignedInteger(value: u32) {
            description("failed to serialize unsigned integer")
            display("failed to serialize unsigned integer: {}", value)
        }

        StringIsNotAscii(string: String) {
            description("string is not ASCII encoded")
            display("string is not ASCII encoded: {}", string)
        }

        StringIsTooLong(string: String) {
            description("string is too long")
            display(
                "string is too long (maximum length is {} bytes): {}",
                u32::max_value(),
                string,
            )
        }

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
