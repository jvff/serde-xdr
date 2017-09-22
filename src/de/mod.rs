use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};
use serde::Deserialize;

use super::errors::{ErrorKind, Result, ResultExt};

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
    pub fn new(reader: &'r mut R) -> Self {
        Deserializer { reader }
    }

    fn deserialize_integer(&mut self, bits: u8) -> Result<i32> {
        let value = self.reader
            .read_i32::<BigEndian>()
            .chain_err(|| ErrorKind::DeserializeInteger(bits))?;

        let most_significant_bit: u32 = 1 << (bits - 1);
        let max_value = (most_significant_bit - 1) as i32;
        let min_value = -max_value - 1;

        ensure!(
            value >= min_value && value <= max_value,
            ErrorKind::InvalidInteger(bits, value)
        );

        Ok(value)
    }

    fn deserialize_unsigned_integer(&mut self, bits: u8) -> Result<u32> {
        let value = self.reader
            .read_u32::<BigEndian>()
            .chain_err(|| ErrorKind::DeserializeInteger(bits))?;

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
        self.reader.read_exact(&mut buffer).chain_err(|| read_data_error_kind)?;
        buffer.truncate(length as usize);

        Ok(buffer)
    }
}

pub fn from_reader<'de, R, T>(reader: &'de mut R) -> Result<T>
where
    R: Read,
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::new(reader);

    Ok(T::deserialize(&mut deserializer)?)
}

mod deserializer;
