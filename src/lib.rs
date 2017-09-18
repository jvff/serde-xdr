#[macro_use]
extern crate error_chain;

mod errors;
mod ser;

pub use errors::{Error, ErrorKind, Result};
pub use ser::{to_bytes, Serializer};
