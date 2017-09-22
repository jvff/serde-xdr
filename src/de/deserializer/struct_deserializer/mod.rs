use byteorder::ReadBytesExt;
use serde::de::{DeserializeSeed, SeqAccess};

use super::super::Deserializer;
use super::super::super::errors::{Error, ErrorKind, Result, ResultExt};

pub struct StructDeserializer<'a, 'de, R>
where
    'de: 'a,
    R: ReadBytesExt + 'de,
{
    name: &'static str,
    fields: &'static [&'static str],
    deserializer: &'a mut Deserializer<'de, R>,
    current_field: usize,
}

impl<'a, 'de, R> StructDeserializer<'a, 'de, R>
where
    'de: 'a,
    R: ReadBytesExt + 'de,
{
    pub fn new(
        name: &'static str,
        fields: &'static [&'static str],
        deserializer: &'a mut Deserializer<'de, R>,
    ) -> Self {
        StructDeserializer {
            name,
            fields,
            deserializer,
            current_field: 0,
        }
    }
}

impl<'a, 'de, R> SeqAccess<'de> for StructDeserializer<'a, 'de, R>
where
    'de: 'a,
    R: ReadBytesExt + 'de,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let value = seed
            .deserialize(&mut *self.deserializer)
            .chain_err(|| {
                ErrorKind::DeserializeStructField(
                    self.name.to_string(),
                    self.fields[self.current_field].to_string(),
                )
            })?;

        self.current_field += 1;

        Ok(Some(value))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.fields.len())
    }
}

#[cfg(test)]
mod tests;
