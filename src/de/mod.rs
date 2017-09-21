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

        let max_value = (1 << (bits - 1)) - 1;
        let min_value = -max_value - 1;

        ensure!(
            value >= min_value && value <= max_value,
            ErrorKind::InvalidInteger(bits, value)
        );

        Ok(value)
    }
}

pub fn from_bytes(_bytes: &[u8]) {}

mod deserializer;
