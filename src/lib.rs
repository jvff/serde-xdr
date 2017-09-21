#![recursion_limit="256"]

extern crate byteorder;
#[macro_use]
extern crate error_chain;
extern crate serde;

#[cfg(test)]
extern crate ordered_float;

mod errors;
mod ser;
mod de;

pub use errors::{Error, ErrorKind, Result};
pub use ser::{to_bytes, Serializer};
pub use de::{from_bytes, Deserializer};
