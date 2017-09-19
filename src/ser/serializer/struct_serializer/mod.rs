use byteorder::WriteBytesExt;
use serde::ser;
use serde::ser::Serialize;

use super::super::Serializer;
use super::super::super::errors::{Error, ErrorKind, Result, ResultExt};

pub struct StructSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    struct_name: &'static str,
    serializer: Option<Serializer<'w, W>>,
}

impl<'w, W> StructSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    pub fn new(
        struct_name: &'static str,
        serializer: Serializer<'w, W>,
    ) -> Self {
        Self {
            struct_name,
            serializer: Some(serializer),
        }
    }
}

impl<'w, W> ser::SerializeStruct for StructSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Serializer<'w, W>;
    type Error = Error;

    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if let Some(serializer) = self.serializer.take() {
            let serializer = value.serialize(serializer)
                .chain_err(|| serialization_error(self.struct_name, key))?;

            self.serializer = Some(serializer);

            Ok(())
        } else {
            Err(fatal_error(self.struct_name))
                .chain_err(|| serialization_error(self.struct_name, key))
        }
    }

    fn end(self) -> Result<Serializer<'w, W>> {
        if let Some(serializer) = self.serializer {
            Ok(serializer)
        } else {
            Err(fatal_error(self.struct_name))
        }
    }
}

fn fatal_error(struct_name: &str) -> Error {
    ErrorKind::SerializeStructFatalError(struct_name.to_string()).into()
}

fn serialization_error(struct_name: &str, field_name: &str) -> ErrorKind {
    ErrorKind::SerializeStructField(
        struct_name.to_string(),
        field_name.to_string(),
    )
}

#[cfg(test)]
mod tests;
