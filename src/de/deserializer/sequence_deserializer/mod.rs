use {
    super::super::{
        errors::{CompatDeserializationError, DeserializationError, Result},
        Deserializer,
    },
    byteorder::ReadBytesExt,
    serde::de::{DeserializeSeed, SeqAccess},
};

pub struct SequenceDeserializer<'a, 'r, 's, R, S>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
    S: ToString + 's,
{
    length: u32,
    type_name: &'s S,
    current_index: u32,
    deserializer: &'a mut Deserializer<'r, R>,
}

impl<'a, 'r, 's, R, S> SequenceDeserializer<'a, 'r, 's, R, S>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
    S: ToString,
{
    pub fn new(
        length: u32,
        type_name: &'s S,
        deserializer: &'a mut Deserializer<'r, R>,
    ) -> Self {
        SequenceDeserializer {
            length,
            type_name,
            deserializer,
            current_index: 0,
        }
    }
}

impl<'a, 'de, 'r, 's, R, S> SeqAccess<'de>
    for SequenceDeserializer<'a, 'r, 's, R, S>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
    S: ToString,
{
    type Error = CompatDeserializationError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.current_index < self.length {
            let value =
                seed.deserialize(&mut *self.deserializer).map_err(|error| {
                    deserialize_error(self.type_name, self.current_index, error)
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

fn deserialize_error<S>(
    type_name: &S,
    index: u32,
    cause: CompatDeserializationError,
) -> DeserializationError
where
    S: ToString,
{
    DeserializationError::failure(
        format!("element {} of type {}", index, type_name.to_string()),
        cause,
    )
}

#[cfg(test)]
mod tests;
