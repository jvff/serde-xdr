use std::fmt;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;

/// Representation of variable-length opaque data.
///
/// Wraps a `Vec<u8>` so that it is serialized and deserialized as a block of
/// contiguous bytes (padded with zeros so that it ends on a 4-byte alignment).
/// With a plain `Vec<u8>`, each byte is serialized as 4 bytes, so it can be
/// quite costly in terms of serialized bytes.
///
/// The [XDR representation](https://tools.ietf.org/html/rfc1014#section-3.9) is
/// a big endian unsigned 32-bit integer with the length of the data followed by
/// the data bytes and ending with a zero byte padding to ensure that the bytes
/// end on a four byte boundary.
///
/// An `OpaqueData` instance can be dereferenced into its internal `Vec<u8>`
/// data, so it can be used just like a normal vector of bytes. It can also be
/// created from an existing `Vec<u8>` and converted into a `Vec<u8>` instance.
///
/// # Examples
///
/// Converting between `OpaqueData` and `Vec<u8>`:
///
/// ```
/// use serde_xdr::OpaqueData;
///
/// let bytes = vec![1, 3, 5, 7, 9];
/// let expected_bytes = bytes.clone();
///
/// let data: OpaqueData = bytes.into();
/// let recovered_bytes: Vec<u8> = data.into();
///
/// assert_eq!(expected_bytes, recovered_bytes);
/// ```
///
/// Using as a `Vec<u8>`:
///
/// ```
/// use serde_xdr::OpaqueData;
///
/// let mut data = OpaqueData::new();
///
/// for i in 0..5 {
///     data.push(2 * i + 1);
/// }
///
/// assert_eq!(data, OpaqueData::from(vec![1, 3, 5, 7, 9]));
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpaqueData {
    data: Vec<u8>,
}

impl OpaqueData {
    /// Create an empty `OpaqueData` representation.
    ///
    /// Even though it is empty, bytes can be added to it later by using it as a
    /// `Vec<u8>`.
    pub fn new() -> Self {
        OpaqueData {
            data: Vec::new(),
        }
    }

    /// Create an `OpaqueData` representation with a given capacity.
    ///
    /// The memory to store the given capacity is allocated but the instance is
    /// initially empty.
    pub fn with_capacity(length: usize) -> Self {
        OpaqueData {
            data: Vec::with_capacity(length),
        }
    }
}

impl From<Vec<u8>> for OpaqueData {
    fn from(bytes: Vec<u8>) -> Self {
        OpaqueData {
            data: bytes,
        }
    }
}

impl Into<Vec<u8>> for OpaqueData {
    fn into(self) -> Vec<u8> {
        self.data
    }
}

impl Deref for OpaqueData {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for OpaqueData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Serialize for OpaqueData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut data = self.data.clone();
        let length = data.len();
        let padding = 3 - (length + 3) % 4;
        let full_length = length + padding;
        let num_blocks = full_length / 4;

        let mut sequence_serializer = serializer.serialize_seq(Some(length))?;

        data.resize(full_length, 0);

        for index in 0..num_blocks {
            let start = 4 * index;
            let end = start + 4;
            let block = &data[start..end];

            let mut block_value = 0 as u32;

            for byte in 0..4 {
                block_value <<= 8;
                block_value |= block[byte] as u32;
            }

            sequence_serializer.serialize_element(&block_value)?;
        }

        sequence_serializer.end()
    }
}

impl<'de> Deserialize<'de> for OpaqueData {
    fn deserialize<D>(deserializer: D) -> Result<OpaqueData, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(OpaqueDataVisitor)
    }
}

struct OpaqueDataVisitor;

impl<'de> Visitor<'de> for OpaqueDataVisitor {
    type Value = OpaqueData;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("variable length opaque data")
    }

    fn visit_seq<V>(self, mut sequence: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let length = sequence.size_hint()
            .ok_or_else(|| {
                de::Error::custom("opaque data length not specied")
            })?;

        let padding = 3 - (length + 3) % 4;
        let full_length = length + padding;
        let num_blocks = full_length / 4;

        let mut data = OpaqueData::with_capacity(full_length as usize);

        for index in 0..num_blocks {
            let block: u32 = sequence.next_element()?
                .ok_or_else(|| {
                    de::Error::invalid_length(4 + index * 4, &self)
                })?;

            data.push(((block >> 24) & 0xff) as u8);
            data.push(((block >> 16) & 0xff) as u8);
            data.push(((block >> 8) & 0xff) as u8);
            data.push((block & 0xff) as u8);
        }

        data.truncate(length);

        Ok(data)
    }
}

#[cfg(test)]
mod tests;
