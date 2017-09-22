use std::fmt;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpaqueData {
    data: Vec<u8>,
}

impl OpaqueData {
    pub fn new() -> Self {
        OpaqueData {
            data: Vec::new(),
        }
    }

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
