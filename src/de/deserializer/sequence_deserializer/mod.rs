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
    type_name: &'static str,
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
        type_name: &'static str,
        deserializer: &'a mut Deserializer<'de, R>,
    ) -> Self {
        SequenceDeserializer {
            length,
            type_name,
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
                    deserialize_error(self.type_name, self.current_index)
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

fn deserialize_error(type_name: &str, index: u32) -> ErrorKind {
    ErrorKind::DeserializeSequenceOrTupleElement(type_name.to_string(), index)
}

#[cfg(test)]
mod tests;
