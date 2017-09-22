use byteorder::ReadBytesExt;
use serde::de::{DeserializeSeed, SeqAccess};

use super::super::Deserializer;
use super::super::super::errors::{Error, ErrorKind, Result, ResultExt};

pub struct SequenceDeserializer<'a, 'de, R>
where
    'de: 'a,
    R: ReadBytesExt + 'de,
{
    length: u32,
    current_index: u32,
    deserializer: &'a mut Deserializer<'de, R>,
}

impl<'a, 'de, R> SequenceDeserializer<'a, 'de, R>
where
    'de: 'a,
    R: ReadBytesExt + 'de,
{
    pub fn new(
        length: u32,
        deserializer: &'a mut Deserializer<'de, R>,
    ) -> Self {
        SequenceDeserializer {
            length,
            deserializer,
            current_index: 0,
        }
    }
}

impl<'a, 'de, R> SeqAccess<'de> for SequenceDeserializer<'a, 'de, R>
where
    'de: 'a,
    R: ReadBytesExt + 'de,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.current_index < self.length {
            let value = seed
                .deserialize(&mut *self.deserializer)
                .chain_err(|| {
                    ErrorKind::DeserializeSequenceElement(self.current_index)
                })?;

            self.current_index += 1;

            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.length as usize)
    }
}

#[cfg(test)]
mod tests;
