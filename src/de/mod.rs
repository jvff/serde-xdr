use std::io::{Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt};
use serde::Deserialize;

use super::errors::{ErrorKind, Result, ResultExt};

pub use self::errors::DeserializationError;

/// Deserializer for the XDR format.
///
/// Structure that holds a mutable borrow of the reader it deserializes data
/// from. It has an implementation of `serde::Deserializer` so that it can
/// deserialize data from its XDR representation.
pub struct Deserializer<'r, R>
where
    R: ReadBytesExt + 'r,
{
    reader: &'r mut R,
}

impl<'r, R> Deserializer<'r, R>
where
    R: ReadBytesExt + 'r,
{
    /// Create a new instance that deserializes data from the given generic
    /// reader.
    pub fn new(reader: &'r mut R) -> Self {
        Deserializer { reader }
    }

    fn deserialize_integer(&mut self, bits: u8) -> Result<i32> {
        let value = self.reader.read_i32::<BigEndian>().chain_err(|| {
            DeserializationError::failure(
                format!("signed {}-bit integer", bits),
            )
        })?;

        let most_significant_bit: u32 = 1 << (bits - 1);
        let max_value = (most_significant_bit - 1) as i32;
        let min_value = -max_value - 1;

        ensure!(
            value >= min_value && value <= max_value,
            DeserializationError::InvalidInteger { bits, value }
        );

        Ok(value)
    }

    fn deserialize_unsigned_integer(&mut self, bits: u8) -> Result<u32> {
        let value = self.reader.read_u32::<BigEndian>().chain_err(|| {
            DeserializationError::failure(
                format!("unsigned {}-bit integer", bits),
            )
        })?;

        let most_significant_bit: u64 = 1 << bits;
        let max_value = (most_significant_bit - 1) as u32;

        ensure!(
            value <= max_value,
            ErrorKind::InvalidUnsignedInteger(bits, value)
        );

        Ok(value)
    }

    fn deserialize_opaque(
        &mut self,
        read_length_error_kind: ErrorKind,
        read_data_error_kind: ErrorKind,
    ) -> Result<Vec<u8>> {
        let length = self.reader
            .read_u32::<BigEndian>()
            .chain_err(|| read_length_error_kind)?;

        let padding_size = 4 - (length + 3) % 4 - 1;
        let buffer_length = length + padding_size;

        let mut buffer = Vec::with_capacity(buffer_length as usize);

        buffer.resize(buffer_length as usize, 0);
        self.reader
            .read_exact(&mut buffer)
            .chain_err(|| read_data_error_kind)?;
        buffer.truncate(length as usize);

        Ok(buffer)
    }
}

/// Deserializes data from a generic reader.
///
/// Deserializes data of a given type `T` from a generic instance that
/// implements `Read`.
///
/// The lifetimes of the deserialized data `'de` and of the reader `'r` are
/// different because the deserializer currently is not zero-copy, which means
/// the returned data owns everything it deserialized.
pub fn from_reader<'de, 'r, R, T>(reader: &'r mut R) -> Result<T>
where
    R: Read,
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::new(reader);

    Ok(T::deserialize(&mut deserializer)?)
}

/// Deserializes data from a slice of bytes.
///
/// Deserializes data of a given type `T` from a generic instance that can be
/// accessed as a reference to a slice of bytes.
///
/// The deserializer is currently zero-copy, which means that the returned data
/// owns everything it deserialized.
pub fn from_bytes<'de, B, T>(bytes: B) -> Result<T>
where
    B: AsRef<[u8]>,
    T: Deserialize<'de>,
{
    let mut reader = Cursor::new(bytes);

    from_reader(&mut reader)
}

mod deserializer;
mod errors;
