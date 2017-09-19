use byteorder::{BigEndian, WriteBytesExt};
use serde::ser;
use serde::ser::Serialize;

use super::Serializer;
use super::super::errors::{Error, ErrorKind, Result, ResultExt};

impl<'w, W> ser::Serializer for &'w mut Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _value: bool) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("bool".to_string()))
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        self.serialize_i32(value as i32)
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        self.serialize_i32(value as i32)
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        self.writer.write_i32::<BigEndian>(value)
            .chain_err(|| ErrorKind::SerializeInteger(value))?;

        Ok(())
    }

    fn serialize_i64(self, _value: i64) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("i64".to_string()))
    }

    fn serialize_u8(self, _value: u8) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("u8".to_string()))
    }

    fn serialize_u16(self, _value: u16) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("u16".to_string()))
    }

    fn serialize_u32(self, _value: u32) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("u32".to_string()))
    }

    fn serialize_u64(self, _value: u64) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("u64".to_string()))
    }

    fn serialize_f32(self, _value: f32) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("f32".to_string()))
    }

    fn serialize_f64(self, _value: f64) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("f64".to_string()))
    }

    fn serialize_char(self, _value: char) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("char".to_string()))
    }

    fn serialize_str(self, _value: &str) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("str".to_string()))
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("str".to_string()))
    }

    fn serialize_none(self) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("none".to_string()))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        bail!(ErrorKind::InvalidDataType("some(?)".to_string()))
    }

    fn serialize_unit(self) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("unit".to_string()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("unit_struct".to_string()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        bail!(ErrorKind::InvalidDataType("unit_variant".to_string()))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        bail!(ErrorKind::InvalidDataType("newtype_struct".to_string()))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        bail!(ErrorKind::InvalidDataType("newtype_variant".to_string()))
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
        _name: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStruct> {
        bail!(ErrorKind::InvalidDataType("struct".to_string()))
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

#[cfg(test)]
mod tests;
