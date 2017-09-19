use std::ascii::AsciiExt;

use byteorder::{BigEndian, WriteBytesExt};
use serde::ser;
use serde::ser::Serialize;

use self::struct_serializer::StructSerializer;
use super::Serializer;
use super::super::errors::{Error, ErrorKind, Result, ResultExt};

impl<'w, W> ser::Serializer for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = StructSerializer<'w, W>;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, value: bool) -> Result<Self> {
        self.serialize_u32(if value { 1 } else { 0 })
    }

    fn serialize_i8(self, value: i8) -> Result<Self> {
        self.serialize_i32(value as i32)
    }

    fn serialize_i16(self, value: i16) -> Result<Self> {
        self.serialize_i32(value as i32)
    }

    fn serialize_i32(self, value: i32) -> Result<Self> {
        self.writer.write_i32::<BigEndian>(value)
            .chain_err(|| ErrorKind::SerializeInteger(value))?;

        Ok(self)
    }

    fn serialize_i64(self, value: i64) -> Result<Self> {
        self.writer.write_i64::<BigEndian>(value)
            .chain_err(|| ErrorKind::SerializeHyperInteger(value))?;

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
            .chain_err(|| ErrorKind::SerializeUnsignedInteger(value))?;

        Ok(self)
    }

    fn serialize_u64(self, value: u64) -> Result<Self> {
        self.writer.write_u64::<BigEndian>(value)
            .chain_err(|| ErrorKind::SerializeUnsignedHyperInteger(value))?;

        Ok(self)
    }

    fn serialize_f32(self, value: f32) -> Result<Self> {
        self.writer.write_f32::<BigEndian>(value)
            .chain_err(|| ErrorKind::SerializeFloat(value))?;

        Ok(self)
    }

    fn serialize_f64(self, value: f64) -> Result<Self> {
        self.writer.write_f64::<BigEndian>(value)
            .chain_err(|| ErrorKind::SerializeDouble(value))?;

        Ok(self)
    }

    fn serialize_char(self, _value: char) -> Result<Self> {
        bail!(ErrorKind::InvalidDataType("char".to_string()))
    }

    fn serialize_str(self, value: &str) -> Result<Self> {
        if value.len() > u32::max_value() as usize {
            bail!(ErrorKind::StringIsTooLong(value.to_string()));
        }

        if !value.is_ascii() {
            bail!(ErrorKind::StringIsNotAscii(value.to_string()));
        }

        let length = value.len();

        let serializer = self.serialize_u32(length as u32)
            .chain_err(|| ErrorKind::SerializeString(value.to_string()))?
            .serialize_bytes(value.as_bytes())
            .chain_err(|| ErrorKind::SerializeString(value.to_string()))?;

        if length % 4 == 0 {
            serializer.serialize_u32(0)
                .chain_err(|| ErrorKind::SerializeString(value.to_string()))
        } else {
            Ok(serializer)
        }
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self> {
        let length = value.len();
        let full_padding = [0u8; 3];
        let padding_size = 4 - (length + 3) % 4 - 1;
        let (padding, _) = full_padding.split_at(padding_size);

        self.writer
            .write_all(value)
            .chain_err(|| ErrorKind::SerializeOpaque(length))?;

        self.writer
            .write_all(padding)
            .chain_err(|| ErrorKind::SerializeOpaque(length))?;

        Ok(self)
    }

    fn serialize_none(self) -> Result<Self> {
        bail!(ErrorKind::InvalidDataType("none".to_string()))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self>
    where
        T: ?Sized + Serialize,
    {
        bail!(ErrorKind::InvalidDataType("some(?)".to_string()))
    }

    fn serialize_unit(self) -> Result<Self> {
        Ok(self)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self> {
        Ok(self)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self> {
        self.serialize_u32(variant_index)
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
            .chain_err(|| ErrorKind::SerializeStruct(name.to_string()))
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
                ErrorKind::SerializeUnionVariant(
                    name.to_string(),
                    variant.to_string(),
                )
            })?;

        value.serialize(serializer)
            .chain_err(|| {
                ErrorKind::SerializeUnionVariant(
                    name.to_string(),
                    variant.to_string(),
                )
            })
    }

    fn serialize_seq(
        self,
        _length: Option<usize>,
    ) -> Result<Self::SerializeSeq> {
        bail!(ErrorKind::InvalidDataType("seq".to_string()))
    }

    fn serialize_tuple(
        self,
        _length: usize,
    ) -> Result<Self::SerializeTuple> {
        bail!(ErrorKind::InvalidDataType("tuple".to_string()))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        bail!(ErrorKind::InvalidDataType("tuple_struct".to_string()))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        bail!(ErrorKind::InvalidDataType("tuple_variant".to_string()))
    }

    fn serialize_map(
        self,
        _length: Option<usize>,
    ) -> Result<Self::SerializeMap> {
        bail!(ErrorKind::InvalidDataType("map".to_string()))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(StructSerializer::new(name, self))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStructVariant> {
        bail!(ErrorKind::InvalidDataType("struct_variant".to_string()))
    }
}

mod struct_serializer;

#[cfg(test)]
mod tests;
