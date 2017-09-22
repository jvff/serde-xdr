#![recursion_limit="256"]

extern crate byteorder;
#[macro_use]
extern crate error_chain;
extern crate serde;

#[cfg(test)]
extern crate ordered_float;
#[cfg(test)] #[macro_use]
extern crate serde_derive;

mod errors;
mod ser;
mod de;
mod opaque_data;

#[cfg(test)]
mod tests;

pub use errors::{Error, ErrorKind, Result};
pub use ser::{to_bytes, to_writer, Serializer};
pub use de::{from_reader, Deserializer};
pub use opaque_data::OpaqueData;
