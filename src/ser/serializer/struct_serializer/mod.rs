use byteorder::WriteBytesExt;
use serde::ser;
use serde::ser::Serialize;

use self::type_name::TypeName;
use super::super::errors::{CompatSerializationError, Result, SerializationError};
use super::super::Serializer;

pub struct StructSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    struct_name: TypeName,
    serializer: Option<Serializer<'w, W>>,
}

impl<'w, W> StructSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    pub fn start_struct(
        struct_name: &'static str,
        serializer: Serializer<'w, W>,
    ) -> Self {
        Self {
            struct_name: TypeName::Struct(struct_name),
            serializer: Some(serializer),
        }
    }

    pub fn start_struct_variant(
        type_name: &'static str,
        variant_name: &'static str,
        serializer: Serializer<'w, W>,
    ) -> Self {
        Self {
            struct_name: TypeName::StructVariant(type_name, variant_name),
            serializer: Some(serializer),
        }
    }

    fn common_serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if let Some(serializer) = self.serializer.take() {
            let serializer = value
                .serialize(serializer)
                .map_err(|_| serialization_error(&self.struct_name, key))?;

            self.serializer = Some(serializer);

            Ok(())
        } else {
            Err(fatal_error(&self.struct_name))
                .map_err(|_| serialization_error(&self.struct_name, key).into())
        }
    }

    fn common_end(self) -> Result<Serializer<'w, W>> {
        if let Some(serializer) = self.serializer {
            Ok(serializer)
        } else {
            Err(fatal_error(&self.struct_name))
        }
    }
}

impl<'w, W> ser::SerializeStruct for StructSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Serializer<'w, W>;
    type Error = CompatSerializationError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.common_serialize_field(key, value)
    }

    fn end(self) -> Result<Serializer<'w, W>> {
        self.common_end()
    }
}

impl<'w, W> ser::SerializeStructVariant for StructSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Serializer<'w, W>;
    type Error = CompatSerializationError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.common_serialize_field(key, value)
    }

    fn end(self) -> Result<Serializer<'w, W>> {
        self.common_end()
    }
}

fn fatal_error(struct_name: &TypeName) -> CompatSerializationError {
    let name = struct_name.to_string();

    SerializationError::StructFatalError { name }.into()
}

fn serialization_error(
    struct_name: &TypeName,
    field_name: &str,
) -> SerializationError {
    SerializationError::Failure {
        what: format!("struct field {}::{}", struct_name, field_name),
    }
}

mod type_name;

#[cfg(test)]
mod tests;
