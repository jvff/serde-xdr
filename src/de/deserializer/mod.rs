use byteorder::{BigEndian, ReadBytesExt};
use serde::de;
use serde::de::Visitor;

use self::enum_deserializer::EnumDeserializer;
use self::sequence_deserializer::SequenceDeserializer;
use self::struct_deserializer::StructDeserializer;
use super::Deserializer;
use super::super::errors::{Error, ErrorKind, Result, ResultExt};

impl<'a, 'r, R> de::Deserializer<'r> for &'a mut Deserializer<'r, R>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
{
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        bail!(ErrorKind::DeserializeUnknownType);
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let value = self.reader
            .read_u32::<BigEndian>()
            .chain_err(|| ErrorKind::DeserializeBool)?;

        match value {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            _ => bail!(ErrorKind::InvalidBool),
        }
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

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let length = self.reader
            .read_u32::<BigEndian>()
            .chain_err(|| ErrorKind::DeserializeSequence)?;

        visitor.visit_seq(SequenceDeserializer::new(length, self))
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
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        visitor.visit_seq(StructDeserializer::new(name, fields, self))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'r>,
    {
        let variant = self.reader
            .read_u32::<BigEndian>()
            .chain_err(|| ErrorKind::DeserializeEnum(name.to_string()))?;
        let variant_name = variants[variant as usize];

        let enum_deserializer =
            EnumDeserializer::new(name, variant, variant_name, self);

        visitor.visit_enum(enum_deserializer)
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

mod enum_deserializer;
mod sequence_deserializer;
mod struct_deserializer;

#[cfg(test)]
mod tests;
