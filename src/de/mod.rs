use byteorder::{BigEndian, ReadBytesExt};

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

    fn deserialize_integer(self, bits: u8) -> Result<i32> {
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

    fn deserialize_unsigned_integer(self, bits: u8) -> Result<u32> {
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
}

pub fn from_bytes(_bytes: &[u8]) {}

mod deserializer;
