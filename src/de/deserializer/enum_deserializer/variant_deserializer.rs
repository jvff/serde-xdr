use byteorder::ReadBytesExt;
use serde::de::{DeserializeSeed, VariantAccess, Visitor};
use serde::Deserializer as SerdeDeserializer;

use super::deserialize_enum_error;
use super::Deserializer;
use super::super::super::super::errors::{Error, Result, ResultExt};

pub struct VariantDeserializer<'a, 'de, R>
where
    'de: 'a,
    R: ReadBytesExt + 'de,
{
    enum_name: &'static str,
    variant_name: &'static str,
    deserializer: &'a mut Deserializer<'de, R>,
}

impl<'a, 'de, R> VariantDeserializer<'a, 'de, R>
where
    'de: 'a,
    R: ReadBytesExt + 'de,
{
    pub fn new(
        enum_name: &'static str,
        variant_name: &'static str,
        deserializer: &'a mut Deserializer<'de, R>,
    ) -> Self {
        VariantDeserializer {
            enum_name,
            variant_name,
            deserializer,
        }
    }
}

impl<'a, 'de, R> VariantAccess<'de> for VariantDeserializer<'a, 'de, R>
where
    'de: 'a,
    R: ReadBytesExt + 'de,
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
