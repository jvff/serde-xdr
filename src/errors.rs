use std::fmt::Display;
use std::io;

use serde::{ser, de};

error_chain! {
    errors {
        Custom(message: String) {
            description("custom error")
            display("{}", message)
        }

        InvalidDataType(type_name: String) {
            description("data type not supported")
            display("data type not supported: {}", type_name)
        }

        SerializeDouble(value: f64) {
            description("failed to serialize double")
            display("failed to serialize double: {}", value)
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

        SerializeOpaque(size: usize) {
            description("failed to serialize opaque")
            display("failed to serialize opaque data of {} bytes", size)
        }

        SerializeString(string: String) {
            description("failed to serialize string")
            display("failed to serialize string: {}", string)
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
