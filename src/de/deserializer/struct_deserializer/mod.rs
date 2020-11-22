use {
    super::super::{
        errors::{CompatDeserializationError, DeserializationError, Result},
        Deserializer,
    },
    byteorder::ReadBytesExt,
    serde::de::{DeserializeSeed, SeqAccess},
};

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
    type Error = CompatDeserializationError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let value =
            seed.deserialize(&mut *self.deserializer).map_err(|error| {
                let struct_name = self.name;
                let field_name = self.fields[self.current_field];

                DeserializationError::failure(
                    format!("struct field {}::{}", struct_name, field_name),
                    error,
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
