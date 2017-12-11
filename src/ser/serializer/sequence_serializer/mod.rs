use byteorder::WriteBytesExt;
use serde::{Serialize, Serializer as SerdeSerializer};
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct,
                 SerializeTupleVariant};

use self::type_name::TypeName;
use super::super::errors::SerializationError;
use super::super::Serializer;
use super::super::super::errors::{Error, ErrorKind, Result, ResultExt};

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

        serializer.serialize_u32(length as u32).chain_err(|| {
            ErrorKind::SerializeFailure(format!("sequence length: {}", length))
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
                .chain_err(|| {
                    serialize_element_error(&self.type_name, self.current_index)
                })?;

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
}

impl<'w, W> SerializeSeq for SequenceSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Serializer<'w, W>;
    type Error = Error;

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
    type Error = Error;

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
    type Error = Error;

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
    type Error = Error;

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

fn serialize_element_error(type_name: &TypeName, index: usize) -> ErrorKind {
    ErrorKind::SerializeFailure(
        format!("element {} of the type {}", index, type_name),
    )
}

mod type_name;

#[cfg(test)]
mod tests;
