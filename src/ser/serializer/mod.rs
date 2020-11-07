use {
    self::{
        sequence_serializer::SequenceSerializer,
        struct_serializer::StructSerializer,
    },
    super::{
        errors::{CompatSerializationError, Result, SerializationError},
        Serializer,
    },
    byteorder::{BigEndian, WriteBytesExt},
    serde::ser::{self, Serialize},
    std::ascii::AsciiExt,
};

impl<'w, W> ser::Serializer for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = CompatSerializationError;

    type SerializeSeq = SequenceSerializer<'w, W>;
    type SerializeTuple = SequenceSerializer<'w, W>;
    type SerializeTupleStruct = SequenceSerializer<'w, W>;
    type SerializeTupleVariant = SequenceSerializer<'w, W>;
    type SerializeMap = Self;
    type SerializeStruct = StructSerializer<'w, W>;
    type SerializeStructVariant = StructSerializer<'w, W>;

    fn serialize_bool(self, value: bool) -> Result<Self> {
        self.serialize_u32(if value { 1 } else { 0 })
            .map_err(|error| Self::serialize_failure("bool", value, error))
    }

    fn serialize_i8(self, value: i8) -> Result<Self> {
        self.serialize_i32(value as i32)
    }

    fn serialize_i16(self, value: i16) -> Result<Self> {
        self.serialize_i32(value as i32)
    }

    fn serialize_i32(self, value: i32) -> Result<Self> {
        self.writer
            .write_i32::<BigEndian>(value)
            .map_err(|error| Self::io_error("integer", value, error))?;

        Ok(self)
    }

    fn serialize_i64(self, value: i64) -> Result<Self> {
        self.writer
            .write_i64::<BigEndian>(value)
            .map_err(|error| Self::io_error("hyper integer", value, error))?;

        Ok(self)
    }

    fn serialize_u8(self, value: u8) -> Result<Self> {
        self.serialize_u32(value as u32)
    }

    fn serialize_u16(self, value: u16) -> Result<Self> {
        self.serialize_u32(value as u32)
    }

    fn serialize_u32(self, value: u32) -> Result<Self> {
        self.writer.write_u32::<BigEndian>(value).map_err(|error| {
            Self::io_error("unsigned integer", value, error)
        })?;

        Ok(self)
    }

    fn serialize_u64(self, value: u64) -> Result<Self> {
        self.writer.write_u64::<BigEndian>(value).map_err(|error| {
            Self::io_error("unsigned hyper integer", value, error)
        })?;

        Ok(self)
    }

    fn serialize_f32(self, value: f32) -> Result<Self> {
        self.writer
            .write_f32::<BigEndian>(value)
            .map_err(|error| Self::io_error("float", value, error))?;

        Ok(self)
    }

    fn serialize_f64(self, value: f64) -> Result<Self> {
        self.writer
            .write_f64::<BigEndian>(value)
            .map_err(|error| Self::io_error("double", value, error))?;

        Ok(self)
    }

    fn serialize_char(self, value: char) -> Result<Self> {
        self.writer
            .write_u32::<BigEndian>(value as u32)
            .map_err(|error| Self::io_error("char", value, error))?;

        Ok(self)
    }

    fn serialize_str(self, value: &str) -> Result<Self> {
        if value.len() > u32::max_value() as usize {
            let string = value.to_string();

            bail!(SerializationError::StringIsTooLong { string });
        }

        if !value.is_ascii() {
            let string = value.to_string();

            bail!(SerializationError::StringIsNotAscii { string });
        }

        self.serialize_bytes(value.as_bytes())
            .map_err(|error| Self::serialize_failure("string", value, error))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self> {
        let length = value.len();

        ensure!(
            length <= u32::max_value() as usize,
            SerializationError::OpaqueDataIsTooLong { length }
        );

        let full_padding = [0u8; 3];
        let padding_size = 4 - (length + 3) % 4 - 1;
        let (padding, _) = full_padding.split_at(padding_size);

        let serializer = self
            .serialize_u32(length as u32)
            .map_err(|error| Self::serialize_opaque_failure(length, error))?;

        serializer
            .writer
            .write_all(value)
            .map_err(|error| Self::serialize_opaque_io_error(length, error))?;

        serializer
            .writer
            .write_all(padding)
            .map_err(|error| Self::serialize_opaque_io_error(length, error))?;

        Ok(serializer)
    }

    fn serialize_none(self) -> Result<Self> {
        self.serialize_u32(0).map_err(|error| {
            Self::serialize_failure("'none'", "optional data", error)
        })
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self>
    where
        T: ?Sized + Serialize,
    {
        let serializer = self.serialize_u32(1).map_err(|error| {
            Self::serialize_failure("'some'", "optional data", error)
        })?;

        value.serialize(serializer).map_err(|error| {
            Self::serialize_failure("'some'", "optional data", error)
        })
    }

    fn serialize_unit(self) -> Result<Self> {
        Ok(self)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self> {
        Ok(self)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self> {
        self.serialize_u32(variant_index).map_err(|error| {
            SerializationError::Failure {
                what: format!("enum variant {}::{}", name, variant),
                cause: Box::new(error.into()),
            }
            .into()
        })
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self).map_err(|error| {
            SerializationError::Failure {
                what: format!("struct {}", name),
                cause: Box::new(error.into()),
            }
            .into()
        })
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self>
    where
        T: ?Sized + Serialize,
    {
        let serializer =
            self.serialize_u32(variant_index).map_err(|error| {
                SerializationError::Failure {
                    what: format!("union variant {}::{}", name, variant),
                    cause: Box::new(error.into()),
                }
            })?;

        value.serialize(serializer).map_err(|error| {
            SerializationError::Failure {
                what: format!("union variant {}::{}", name, variant),
                cause: Box::new(error.into()),
            }
            .into()
        })
    }

    fn serialize_seq(
        self,
        length: Option<usize>,
    ) -> Result<Self::SerializeSeq> {
        Ok(SequenceSerializer::start_sequence(length, self)?)
    }

    fn serialize_tuple(self, _length: usize) -> Result<Self::SerializeTuple> {
        Ok(SequenceSerializer::start_tuple(self))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(SequenceSerializer::start_tuple_struct(name, self))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let serializer =
            self.serialize_u32(variant_index).map_err(|error| {
                SerializationError::Failure {
                    what: format!("tuple variant {}::{}", name, variant),
                    cause: Box::new(error.into()),
                }
            })?;

        Ok(SequenceSerializer::start_tuple_variant(
            name, variant, serializer,
        ))
    }

    fn serialize_map(
        self,
        _length: Option<usize>,
    ) -> Result<Self::SerializeMap> {
        bail!(SerializationError::MapIsNotSupported)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(StructSerializer::start_struct(name, self))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStructVariant> {
        let serializer =
            self.serialize_u32(variant_index).map_err(|error| {
                SerializationError::Failure {
                    what: format!("struct variant {}::{}", name, variant),
                    cause: Box::new(error.into()),
                }
            })?;

        Ok(StructSerializer::start_struct_variant(
            name, variant, serializer,
        ))
    }
}

mod sequence_serializer;
mod struct_serializer;

#[cfg(test)]
mod tests;
