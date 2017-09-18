use std::fmt::Display;

use serde::{ser, de};

error_chain! {
    errors {
        Custom(message: String) {
            description("custom error")
            display("{}", message)
        }
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
