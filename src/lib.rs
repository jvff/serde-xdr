//! XDR serialization and deserialization for Serde.
//!
//! This crate implements serialization to and deserialization from the External
//! Data Representation Standard ([XDR][1]) using the [Serde][2] serialization
//! and deserialization framework.
//!
//! Usage is mainly through the helper functions:
//!
//! - [`serde_xdr::ser::to_bytes`](fn.to_bytes.html)
//! - [`serde_xdr::ser::to_writer`](fn.to_writer.html)
//! - [`serde_xdr::de::from_reader`](fn.from_reader.html)
//!
//! [1]: https://tools.ietf.org/html/rfc1014
//! [2]: https://serde.rs
//!
//! # Examples
//!
//! ```
//! extern crate serde_xdr;
//! extern crate serde_bytes;
//! #[macro_use]
//! extern crate serde_derive;
//!
//! use std::io::Cursor;
//!
//! use serde_bytes::ByteBuf;
//! use serde_xdr::{from_reader, to_writer};
//!
//! #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
//! enum FileType {
//!     Text,
//!     Data(String),
//!     Exec(String),
//! }
//!
//! #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
//! struct File {
//!     filename: String,
//!     filetype: FileType,
//!     owner: String,
//!     data: ByteBuf,
//! }
//!
//! fn main() {
//!     let file_contents: Vec<u8> = "(quit)".as_bytes().into();
//!
//!     let initial_file = File {
//!         filename: "sillyprog".to_string(),
//!         filetype: FileType::Exec("lisp".to_string()),
//!         owner: "john".to_string(),
//!         data: file_contents.into(),
//!     };
//!
//!     // Serialize
//!     let mut bytes = Vec::new();
//!
//!     to_writer(&mut bytes, &initial_file).unwrap();
//!
//!     // Deserialize
//!     let mut cursor = Cursor::new(bytes);
//!
//!     let recovered_file = from_reader(&mut cursor).unwrap();
//!
//!     assert_eq!(initial_file, recovered_file);
//! }
//! ```

#![recursion_limit="256"]
#![deny(missing_docs)]

extern crate byteorder;
#[macro_use]
extern crate error_chain;
extern crate serde;

#[cfg(test)]
extern crate ordered_float;
#[cfg(test)]
extern crate serde_bytes;
#[cfg(test)] #[macro_use]
extern crate serde_derive;

#[allow(missing_docs)]
mod errors;
mod ser;
mod de;

#[cfg(test)]
mod tests;

/// Serialization and deserialization functions for opaque data.
pub mod opaque_data;

pub use errors::{Error, ErrorKind, Result};
pub use ser::{to_bytes, to_writer, Serializer};
pub use de::{from_reader, Deserializer};
