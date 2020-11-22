use std::fmt;
use std::fmt::Formatter;
use std::iter;
use std::marker::PhantomData;

use serde::de::{Deserializer, Error, SeqAccess, Visitor};
use serde::ser::{SerializeTuple, Serializer};

use super::byte_array::ByteArray;

/// Serialize a slice of bytes as opaque data with a known fixed length.
pub fn serialize<T, S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    let bytes = bytes.as_ref();
    let length = bytes.len();
    let num_blocks = (length + 3) / 4;

    let mut tuple_serializer = serializer.serialize_tuple(num_blocks)?;

    let mut padded_bytes = bytes.iter().cloned().chain(iter::repeat(0u8));

    for _ in 0..num_blocks {
        let mut block = 0u32;

        for _ in 0..4 {
            let byte = padded_bytes
                .next()
                .expect("infinite iterator should always return a value");

            block <<= 8;
            block |= byte as u32;
        }

        tuple_serializer.serialize_element(&block)?;
    }

    tuple_serializer.end()
}

/// Deserialize opaque data with a known fixed length into a vector of bytes.
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: ByteArray,
{
    let length = T::len();
    let num_blocks = (length + 3) / 4;

    deserializer.deserialize_tuple(num_blocks, ByteArrayVisitor::new())
}

struct ByteArrayVisitor<T>
where
    T: ByteArray,
{
    _byte_array: PhantomData<T>,
}

impl<T> ByteArrayVisitor<T>
where
    T: ByteArray,
{
    pub fn new() -> Self {
        ByteArrayVisitor {
            _byte_array: PhantomData,
        }
    }

    fn visit_full_block<'de, A>(
        &self,
        block_index: usize,
        bytes: &mut [u8],
        sequence: &mut A,
    ) -> Result<(), A::Error>
    where
        A: SeqAccess<'de>,
    {
        if let Some(block) = sequence.next_element::<u32>()? {
            let start = block_index * 4;
            let end = start + 4;

            decode_full_block(block, &mut bytes[start..end]);

            Ok(())
        } else {
            let blocks_read = block_index;
            let bytes_read = blocks_read * 4;

            Err(A::Error::invalid_length(bytes_read, self))
        }
    }

    fn visit_partial_block<'de, A>(
        &self,
        total_blocks: usize,
        num_bytes: usize,
        bytes: &mut [u8],
        sequence: &mut A,
    ) -> Result<(), A::Error>
    where
        A: SeqAccess<'de>,
    {
        if let Some(block) = sequence.next_element::<u32>()? {
            let start = (total_blocks - 1) * 4;

            decode_partial_block(block, &mut bytes[start..], num_bytes);

            Ok(())
        } else {
            let blocks_read = total_blocks - 1;
            let bytes_read = blocks_read * 4;

            Err(A::Error::invalid_length(bytes_read, self))
        }
    }
}

impl<'de, T> Visitor<'de> for ByteArrayVisitor<T>
where
    T: ByteArray,
{
    type Value = T;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "opaque data with fixed length of {} bytes",
            T::len()
        )
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut byte_array = T::default();
        let length = T::len();
        let num_blocks = (length + 3) / 4;

        {
            let bytes = byte_array.as_mut();

            for block_index in 0..(num_blocks - 1) {
                self.visit_full_block(block_index, bytes, &mut sequence)?;
            }

            let bytes_read = (num_blocks - 1) * 4;
            let remaining_bytes = length - bytes_read;

            self.visit_partial_block(
                num_blocks,
                remaining_bytes,
                bytes,
                &mut sequence,
            )?;
        }

        Ok(byte_array)
    }
}

fn decode_full_block(block: u32, bytes: &mut [u8]) {
    bytes[0] = ((block >> 24) & 0xff) as u8;
    bytes[1] = ((block >> 16) & 0xff) as u8;
    bytes[2] = ((block >> 8) & 0xff) as u8;
    bytes[3] = (block & 0xff) as u8;
}

fn decode_partial_block(mut block: u32, bytes: &mut [u8], num_bytes: usize) {
    let max_index = num_bytes - 1;
    let padding_bytes = 4 - num_bytes;
    let padding_bits = 8 * padding_bytes;

    block >>= padding_bits;

    for byte_in_block in 0..num_bytes {
        let index = max_index - byte_in_block;

        bytes[index] = (block & 0xff) as u8;

        block >>= 8;
    }
}
