use byteorder::ReadBytesExt;
use serde::de::{DeserializeSeed, SeqAccess};

use super::super::Deserializer;
use super::super::errors::DeserializationError;
use super::super::super::errors::{Error, Result, ResultExt};

pub struct StructDeserializer<'a, 'r, R>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
{
    name: &'static str,
    fields: &'static [&'static str],
    deserializer: &'a mut Deserializer<'r, R>,
    current_field: usize,
}

impl<'a, 'r, R> StructDeserializer<'a, 'r, R>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
{
    pub fn new(
        name: &'static str,
        fields: &'static [&'static str],
        deserializer: &'a mut Deserializer<'r, R>,
    ) -> Self {
        StructDeserializer {
            name,
            fields,
            deserializer,
            current_field: 0,
        }
    }
}

impl<'a, 'de, 'r, R> SeqAccess<'de> for StructDeserializer<'a, 'r, R>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(&mut *self.deserializer).chain_err(|| {
            let struct_name = self.name;
            let field_name = self.fields[self.current_field];

            DeserializationError::failure(
                format!("struct field {}::{}", struct_name, field_name),
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
