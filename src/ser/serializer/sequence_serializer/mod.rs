use byteorder::WriteBytesExt;
use serde::ser::{
    SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer as SerdeSerializer};

use self::type_name::TypeName;
use super::super::errors::{
    CompatSerializationError, Result, SerializationError,
};
use super::super::Serializer;

pub struct SequenceSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type_name: TypeName,
    serializer: Option<Serializer<'w, W>>,
    current_index: usize,
}

impl<'w, W> SequenceSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    pub fn new(type_name: TypeName, serializer: Serializer<'w, W>) -> Self {
        SequenceSerializer {
            type_name,
            serializer: Some(serializer),
            current_index: 0,
        }
    }

    pub fn start_sequence(
        length: Option<usize>,
        mut serializer: Serializer<'w, W>,
    ) -> Result<Self> {
        if let Some(length) = length {
            serializer = Self::serialize_length(length, serializer)?;
        } else {
            bail!(SerializationError::SequenceWithUnknownLength);
        }

        Ok(SequenceSerializer::new(TypeName::Sequence, serializer))
    }

    pub fn start_tuple(serializer: Serializer<'w, W>) -> Self {
        SequenceSerializer::new(TypeName::Tuple, serializer)
    }

    pub fn start_tuple_struct(
        name: &'static str,
        serializer: Serializer<'w, W>,
    ) -> Self {
        SequenceSerializer::new(TypeName::TupleStruct(name), serializer)
    }

    pub fn start_tuple_variant(
        type_name: &'static str,
        variant_name: &'static str,
        serializer: Serializer<'w, W>,
    ) -> Self {
        SequenceSerializer::new(
            TypeName::TupleVariant(type_name, variant_name),
            serializer,
        )
    }

    fn serialize_length(
        length: usize,
        serializer: Serializer<'w, W>,
    ) -> Result<Serializer<'w, W>> {
        Self::ensure_length_is_valid(length)?;

        serializer.serialize_u32(length as u32).map_err(|error| {
            SerializationError::Failure {
                what: format!("sequence length: {}", length),
                cause: Box::new(error.into()),
            }
            .into()
        })
    }

    fn ensure_length_is_valid(length: usize) -> Result<()> {
        let max_length = u32::max_value() as usize;

        ensure!(
            length <= max_length,
            SerializationError::SequenceTooLong { length }
        );

        Ok(())
    }

    fn common_serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if let Some(serializer) = self.serializer.take() {
            let serializer = value
                .serialize(serializer)
                .map_err(|error| self.failure(error))?;

            self.current_index += 1;
            self.serializer = Some(serializer);

            Ok(())
        } else {
            bail!(fatal_error(&self.type_name));
        }
    }

    fn common_end(self) -> Result<Serializer<'w, W>> {
        if let Some(serializer) = self.serializer {
            Ok(serializer)
        } else {
            bail!(fatal_error(&self.type_name))
        }
    }

    fn failure<E>(&self, error: E) -> SerializationError
    where
        E: Into<CompatSerializationError>,
    {
        let index = self.current_index;
        let type_name = &self.type_name;

        SerializationError::Failure {
            what: format!("element {} of the type {}", index, type_name),
            cause: Box::new(error.into()),
        }
    }
}

impl<'w, W> SerializeSeq for SequenceSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Serializer<'w, W>;
    type Error = CompatSerializationError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.common_serialize_element(value)
    }

    fn end(self) -> Result<Serializer<'w, W>> {
        self.common_end()
    }
}

impl<'w, W> SerializeTuple for SequenceSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Serializer<'w, W>;
    type Error = CompatSerializationError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.common_serialize_element(value)
    }

    fn end(self) -> Result<Serializer<'w, W>> {
        self.common_end()
    }
}

impl<'w, W> SerializeTupleStruct for SequenceSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Serializer<'w, W>;
    type Error = CompatSerializationError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.common_serialize_element(value)
    }

    fn end(self) -> Result<Serializer<'w, W>> {
        self.common_end()
    }
}

impl<'w, W> SerializeTupleVariant for SequenceSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Serializer<'w, W>;
    type Error = CompatSerializationError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.common_serialize_element(value)
    }

    fn end(self) -> Result<Serializer<'w, W>> {
        self.common_end()
    }
}

fn fatal_error(type_name: &TypeName) -> SerializationError {
    let type_name = type_name.to_string();

    SerializationError::SequenceOrTupleFatalError { type_name }
}

mod type_name;

#[cfg(test)]
mod tests;
