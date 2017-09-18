extern crate byteorder;
#[macro_use]
extern crate error_chain;
extern crate serde;

mod errors;
mod ser;
mod de;

pub use errors::{Error, ErrorKind, Result};
pub use ser::{to_bytes, Serializer};
pub use de::{from_bytes, Deserializer};
