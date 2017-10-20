use std::ascii::AsciiExt;

use byteorder::{BigEndian, WriteBytesExt};
use serde::ser;
use serde::ser::Serialize;

use self::sequence_serializer::SequenceSerializer;
use self::struct_serializer::StructSerializer;
use super::Serializer;
use super::super::errors::{Error, ErrorKind, Result, ResultExt};

impl<'w, W> ser::Serializer for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    type SerializeSeq = SequenceSerializer<'w, W>;
    type SerializeTuple = SequenceSerializer<'w, W>;
    type SerializeTupleStruct = SequenceSerializer<'w, W>;
    type SerializeTupleVariant = SequenceSerializer<'w, W>;
    type SerializeMap = Self;
    type SerializeStruct = StructSerializer<'w, W>;
    type SerializeStructVariant = StructSerializer<'w, W>;

    fn serialize_bool(self, value: bool) -> Result<Self> {
        self.serialize_u32(if value { 1 } else { 0 })
            .chain_err(|| Self::serialize_failure("bool", value))
    }

    fn serialize_i8(self, value: i8) -> Result<Self> {
        self.serialize_i32(value as i32)
    }

    fn serialize_i16(self, value: i16) -> Result<Self> {
        self.serialize_i32(value as i32)
    }

    fn serialize_i32(self, value: i32) -> Result<Self> {
        self.writer.write_i32::<BigEndian>(value)
            .chain_err(|| Self::serialize_failure("integer", value))?;

        Ok(self)
    }

    fn serialize_i64(self, value: i64) -> Result<Self> {
        self.writer.write_i64::<BigEndian>(value)
            .chain_err(|| Self::serialize_failure("hyperinteger", value))?;

        Ok(self)
    }

    fn serialize_u8(self, value: u8) -> Result<Self> {
        self.serialize_u32(value as u32)
    }

    fn serialize_u16(self, value: u16) -> Result<Self> {
        self.serialize_u32(value as u32)
    }

    fn serialize_u32(self, value: u32) -> Result<Self> {
        self.writer.write_u32::<BigEndian>(value)
            .chain_err(|| Self::serialize_failure("unsignedinteger", value))?;

        Ok(self)
    }

    fn serialize_u64(self, value: u64) -> Result<Self> {
        self.writer.write_u64::<BigEndian>(value)
            .chain_err(|| Self::serialize_failure("unsignedhyperinteger", value))?;

        Ok(self)
    }

    fn serialize_f32(self, value: f32) -> Result<Self> {
        self.writer.write_f32::<BigEndian>(value)
            .chain_err(|| Self::serialize_failure("float", value))?;

        Ok(self)
    }

    fn serialize_f64(self, value: f64) -> Result<Self> {
        self.writer.write_f64::<BigEndian>(value)
            .chain_err(|| Self::serialize_failure("double", value))?;

        Ok(self)
    }

    fn serialize_char(self, value: char) -> Result<Self> {
        self.writer.write_u32::<BigEndian>(value as u32)
            .chain_err(|| Self::serialize_failure("char", value))?;

        Ok(self)
    }

    fn serialize_str(self, value: &str) -> Result<Self> {
        if value.len() > u32::max_value() as usize {
            bail!(ErrorKind::StringIsTooLong(value.to_string()));
        }

        if !value.is_ascii() {
            bail!(ErrorKind::StringIsNotAscii(value.to_string()));
        }

        self.serialize_bytes(value.as_bytes())
            .chain_err(|| Self::serialize_failure("string", value))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self> {
        let length = value.len();

        ensure!(
            length <= u32::max_value() as usize,
            ErrorKind::OpaqueDataIsTooLong(length)
        );

        let full_padding = [0u8; 3];
        let padding_size = 4 - (length + 3) % 4 - 1;
        let (padding, _) = full_padding.split_at(padding_size);

        let serializer = self.serialize_u32(length as u32)
            .chain_err(|| Self::serialize_opaque_failure(length))?;

        serializer.writer
            .write_all(value)
            .chain_err(|| Self::serialize_opaque_failure(length))?;

        serializer.writer
            .write_all(padding)
            .chain_err(|| Self::serialize_opaque_failure(length))?;

        Ok(serializer)
    }

    fn serialize_none(self) -> Result<Self> {
        self.serialize_u32(0)
            .chain_err(|| Self::serialize_failure("'none'", "optional data"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self>
    where
        T: ?Sized + Serialize,
    {
        let serializer = self.serialize_u32(1)
            .chain_err(|| Self::serialize_failure("'some'", "optional data"))?;

        value.serialize(serializer)
            .chain_err(|| Self::serialize_failure("'some'", "optional data"))
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
        self.serialize_u32(variant_index)
            .chain_err(|| {
                ErrorKind::SerializeFailure(
                    format!("enum variant {}::{}", name, variant),
                )
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
        value.serialize(self)
            .chain_err(|| {
                ErrorKind::SerializeFailure(format!("struct {}", name))
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
        let serializer = self.serialize_u32(variant_index)
            .chain_err(|| {
                ErrorKind::SerializeFailure(
                    format!("union variant {}::{}", name, variant),
                )
            })?;

        value.serialize(serializer)
            .chain_err(|| {
                ErrorKind::SerializeFailure(
                    format!("union variant {}::{}", name, variant),
                )
            })
    }

    fn serialize_seq(
        self,
        length: Option<usize>,
    ) -> Result<Self::SerializeSeq> {
        Ok(SequenceSerializer::start_sequence(length, self)?)
    }

    fn serialize_tuple(
        self,
        _length: usize,
    ) -> Result<Self::SerializeTuple> {
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
        _variant_index: u32,
        variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SequenceSerializer::start_tuple_variant(name, variant, self))
    }

    fn serialize_map(
        self,
        _length: Option<usize>,
    ) -> Result<Self::SerializeMap> {
        bail!(ErrorKind::MapIsNotSupported)
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
        _variant_index: u32,
        variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(StructSerializer::start_struct_variant(name, variant, self))
    }
}

mod sequence_serializer;
mod struct_serializer;

#[cfg(test)]
mod tests;
