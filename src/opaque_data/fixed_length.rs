use std::iter;

use serde::ser::{Serializer, SerializeTuple};

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
