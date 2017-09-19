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

        SerializeInteger(value: i32) {
            description("failed to serialize integer")
            display("failed to serialize integer: {}", value)
        }

        SerializeUnsignedInteger(value: u32) {
            description("failed to serialize unsigned integer")
            display("failed to serialize unsigned integer: {}", value)
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
