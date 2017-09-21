use byteorder::{BigEndian, ReadBytesExt};
use serde::de;
use serde::de::Visitor;

use super::Deserializer;
use super::super::errors::{Error, ErrorKind, Result, ResultExt};

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

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let value = self.reader
            .read_i64::<BigEndian>()
            .chain_err(|| ErrorKind::DeserializeInteger(64))?;

        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let value = self.deserialize_unsigned_integer(8)?;

        visitor.visit_u8(value as u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let value = self.deserialize_unsigned_integer(16)?;

        visitor.visit_u16(value as u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let value = self.deserialize_unsigned_integer(32)?;

        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let value = self.reader
            .read_u64::<BigEndian>()
            .chain_err(|| ErrorKind::DeserializeUnsignedInteger(64))?;

        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let value = self.reader
            .read_f32::<BigEndian>()
            .chain_err(|| ErrorKind::DeserializeFloat)?;

        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let value = self.reader
            .read_f64::<BigEndian>()
            .chain_err(|| ErrorKind::DeserializeDouble)?;

        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::InvalidDataType("char".to_string()));
    }

    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let buffer = self.deserialize_opaque(
            ErrorKind::DeserializeString,
            ErrorKind::DeserializeString,
        )?;

        if buffer.len() % 4 == 0 {
            let mut extra_padding = [0; 4];

            self.reader
                .read_exact(&mut extra_padding)
                .chain_err(|| ErrorKind::DeserializeString)?;
        }

        let string = String::from_utf8(buffer)
            .chain_err(|| ErrorKind::DeserializeString)?;

        visitor.visit_string(string)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let buffer = self.deserialize_opaque(
            ErrorKind::DeserializeOpaque,
            ErrorKind::DeserializeOpaque,
        )?;

        visitor.visit_byte_buf(buffer)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let option = self.reader
            .read_i32::<BigEndian>()
            .chain_err(|| ErrorKind::DeserializeOption)?;

        let result = match option {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => bail!(ErrorKind::InvalidOption),
        };

        result.chain_err(|| ErrorKind::DeserializeOption)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        visitor.visit_newtype_struct(self)
            .chain_err(|| ErrorKind::DeserializeStruct(name.to_string()))
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
