use byteorder::ReadBytesExt;
use serde::de;
use serde::de::Visitor;

use super::Deserializer;
use super::super::errors::{Error, ErrorKind, Result};

impl<'r, R> de::Deserializer<'r> for Deserializer<'r, R>
where
    R: ReadBytesExt + 'r,
{
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::DeserializeUnknownType);
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("bool".to_string()));
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let value = self.deserialize_integer(8)?;

        visitor.visit_i8(value as i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let value = self.deserialize_integer(16)?;

        visitor.visit_i16(value as i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let value = self.deserialize_integer(32)?;

        visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("i64".to_string()));
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("u8".to_string()));
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("u16".to_string()));
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("u32".to_string()));
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("u64".to_string()));
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("f32".to_string()));
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("f64".to_string()));
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("char".to_string()));
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("str".to_string()));
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("string".to_string()));
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("bytes".to_string()));
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("byte_buf".to_string()));
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("option".to_string()));
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("unit".to_string()));
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("unit_struct".to_string()));
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("newtype_struct".to_string()));
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("seq".to_string()));
    }

    fn deserialize_tuple<V>(
        self,
        _length: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("tuple".to_string()));
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _length: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("tuple_struct".to_string()));
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("map".to_string()));
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("struct".to_string()));
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("enum".to_string()));
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("identifier".to_string()));
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::DeserializeUnknownType);
    }
}

#[cfg(test)]
mod tests;
