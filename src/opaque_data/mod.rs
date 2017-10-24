/// Serialization and deserialization of a sequence of bytes as opaque data with
/// a known fixed length.
///
/// This module contains a `serialize` function that can be specified to be used
/// to serialize a field with the `serialize_with` serde attribute.
///
/// # Examples
///
/// ```
/// extern crate serde_xdr;
/// #[macro_use]
/// extern crate serde_derive;
///
/// #[derive(Serialize)]
/// struct Data {
///     #[serde(serialize_with = "serde_xdr::opaque_data::fixed_length::serialize")]
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
/// }
/// ```
pub mod fixed_length;

#[cfg(test)]
mod tests;
