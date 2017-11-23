use std::char;

use byteorder::{BigEndian, ReadBytesExt};
use serde::de;
use serde::de::Visitor;

use self::enum_deserializer::EnumDeserializer;
use self::sequence_deserializer::SequenceDeserializer;
use self::struct_deserializer::StructDeserializer;
use super::Deserializer;
use super::errors::DeserializationError;
use super::super::errors::{Error, ErrorKind, Result, ResultExt};

impl<'a, 'de, 'r, R> de::Deserializer<'de> for &'a mut Deserializer<'r, R>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
{
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        bail!(ErrorKind::DeserializeUnknownType);
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.reader
            .read_u32::<BigEndian>()
            .chain_err(|| DeserializationError::failure("bool"))?;

        match value {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            raw_value => bail!(ErrorKind::InvalidBool(raw_value)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.deserialize_integer(8)?;

        visitor.visit_i8(value as i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.deserialize_integer(16)?;

        visitor.visit_i16(value as i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.deserialize_integer(32)?;

        visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.reader
            .read_i64::<BigEndian>()
            .chain_err(
                || DeserializationError::failure("signed 64-bit integer"),
            )?;

        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.deserialize_unsigned_integer(8)?;

        visitor.visit_u8(value as u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.deserialize_unsigned_integer(16)?;

        visitor.visit_u16(value as u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.deserialize_unsigned_integer(32)?;

        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.reader
            .read_u64::<BigEndian>()
            .chain_err(
                || DeserializationError::failure("unsigned 64-bit integer"),
            )?;

        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.reader
            .read_f32::<BigEndian>()
            .chain_err(|| DeserializationError::failure("float"))?;

        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.reader
            .read_f64::<BigEndian>()
            .chain_err(|| DeserializationError::failure("double"))?;

        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let raw_value = self.reader
            .read_u32::<BigEndian>()
            .chain_err(|| DeserializationError::failure("char"))?;

        let value = char::from_u32(raw_value)
            .ok_or_else(|| ErrorKind::InvalidChar(raw_value))?;

        visitor.visit_char(value as char)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let buffer = self.deserialize_opaque(
            DeserializationError::failure("string").into(),
            DeserializationError::failure("string").into(),
        )?;

        let string = String::from_utf8(buffer)
            .chain_err(|| DeserializationError::failure("string"))?;

        visitor.visit_string(string)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let buffer = self.deserialize_opaque(
            DeserializationError::failure("opaque").into(),
            DeserializationError::failure("opaque").into(),
        )?;

        visitor.visit_byte_buf(buffer)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let option = self.reader
            .read_i32::<BigEndian>()
            .chain_err(|| DeserializationError::failure("option"))?;

        let result = match option {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => bail!(ErrorKind::InvalidOption),
        };

        result.chain_err(|| DeserializationError::failure("option"))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self).chain_err(
            || DeserializationError::failure(format!("struct {}", name)),
        )
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let length = self.reader
            .read_u32::<BigEndian>()
            .chain_err(|| DeserializationError::failure("sequence"))?;

        visitor.visit_seq(SequenceDeserializer::new(length, &"sequence", self))
    }

    fn deserialize_tuple<V>(self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if length > u32::max_value() as usize {
            bail!(ErrorKind::TupleHasTooManyElements(length));
        }

        let length = length as u32;

        visitor.visit_seq(SequenceDeserializer::new(length, &"tuple", self))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        length: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if length > u32::max_value() as usize {
            bail!(ErrorKind::TupleHasTooManyElements(length));
        }

        let length = length as u32;
        let type_name = format!("tuple struct {}", name);

        visitor.visit_seq(SequenceDeserializer::new(length, &type_name, self))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        bail!(ErrorKind::MapIsNotSupported);
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
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
        V: Visitor<'de>,
    {
        let variant = self.reader
            .read_u32::<BigEndian>()
            .chain_err(
                || DeserializationError::failure(format!("enum {}", name)),
            )?;
        let variant_name = variants[variant as usize];

        let enum_deserializer =
            EnumDeserializer::new(name, variant, variant_name, self);

        visitor.visit_enum(enum_deserializer)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        bail!(ErrorKind::IdentifierNotSupported);
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        bail!(ErrorKind::DeserializeUnknownType);
    }
}

mod enum_deserializer;
mod sequence_deserializer;
mod struct_deserializer;

#[cfg(test)]
mod tests;
