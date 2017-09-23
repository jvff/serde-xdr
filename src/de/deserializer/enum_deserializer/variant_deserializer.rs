use byteorder::ReadBytesExt;
use serde::de::{DeserializeSeed, VariantAccess, Visitor};
use serde::Deserializer as SerdeDeserializer;

use super::deserialize_enum_error;
use super::Deserializer;
use super::super::super::super::errors::{Error, Result, ResultExt};

pub struct VariantDeserializer<'a, 'r, R>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
{
    enum_name: &'static str,
    variant_name: &'static str,
    deserializer: &'a mut Deserializer<'r, R>,
}

impl<'a, 'r, R> VariantDeserializer<'a, 'r, R>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
{
    pub fn new(
        enum_name: &'static str,
        variant_name: &'static str,
        deserializer: &'a mut Deserializer<'r, R>,
    ) -> Self {
        VariantDeserializer {
            enum_name,
            variant_name,
            deserializer,
        }
    }
}

impl<'a, 'de, 'r, R> VariantAccess<'de> for VariantDeserializer<'a, 'r, R>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
{
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>
    {
        seed.deserialize(&mut *self.deserializer)
            .chain_err(|| {
                deserialize_enum_error(self.enum_name, self.variant_name)
            })
    }

    fn tuple_variant<V>(self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserializer
            .deserialize_tuple(length, visitor)
            .chain_err(|| {
                deserialize_enum_error(self.enum_name, self.variant_name)
            })
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserializer
            .deserialize_struct(self.variant_name, fields, visitor)
            .chain_err(|| {
                deserialize_enum_error(self.enum_name, self.variant_name)
            })
    }
}
