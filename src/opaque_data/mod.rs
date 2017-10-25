/// Serialization and deserialization of a sequence of bytes as opaque data with
/// a known fixed length.
///
/// This module contains `serialize` and `deserialize` functions that can be
/// specified to be used to serialize a field using the `with` serde attribute.
///
/// # Examples
///
/// ```
/// extern crate serde_xdr;
/// #[macro_use]
/// extern crate serde_derive;
///
/// use std::io::Cursor;
///
/// #[derive(Debug, Serialize, Deserialize, PartialEq)]
/// struct Data {
///     #[serde(with = "serde_xdr::opaque_data::fixed_length")]
///     bytes: [u8; 8],
/// }
///
/// fn main() {
///     let fibonacci = Data {
///         bytes: [1, 1, 2, 3, 5, 8, 13, 21],
///     };
///
///     let bytes = serde_xdr::to_bytes(&fibonacci).unwrap();
///
///     assert_eq!(bytes, vec![1, 1, 2, 3, 5, 8, 13, 21]);
///
///     let mut cursor = Cursor::new(bytes);
///     let deserialized_data = serde_xdr::from_reader(&mut cursor).unwrap();
///
///     assert_eq!(fibonacci, deserialized_data);
/// }
/// ```
pub mod fixed_length;

mod byte_array;

#[cfg(test)]
mod tests;
