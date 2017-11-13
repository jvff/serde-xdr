use byteorder::ReadBytesExt;
use serde::de::{DeserializeSeed, EnumAccess, IntoDeserializer};
use serde::de::value::U32Deserializer;

use self::variant_deserializer::VariantDeserializer;
use super::Deserializer;
use super::super::super::errors::{Error, ErrorKind, Result, ResultExt};

pub struct EnumDeserializer<'a, 'r, R>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
{
    enum_name: &'static str,
    variant: u32,
    variant_name: &'static str,
    deserializer: &'a mut Deserializer<'r, R>,
}

impl<'a, 'r, R> EnumDeserializer<'a, 'r, R>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
{
    pub fn new(
        enum_name: &'static str,
        variant: u32,
        variant_name: &'static str,
        deserializer: &'a mut Deserializer<'r, R>,
    ) -> Self {
        EnumDeserializer {
            enum_name,
            variant,
            variant_name,
            deserializer,
        }
    }
}

impl<'a, 'de, 'r, R> EnumAccess<'de> for EnumDeserializer<'a, 'r, R>
where
    'r: 'a,
    R: ReadBytesExt + 'r,
{
    type Error = Error;
    type Variant = VariantDeserializer<'a, 'r, R>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let variant_code_deserializer: U32Deserializer<Error> =
            self.variant.into_deserializer();

        let value = seed.deserialize(variant_code_deserializer)
            .chain_err(
                || deserialize_enum_error(self.enum_name, self.variant_name),
            )?;

        let variant_deserializer = VariantDeserializer::new(
            self.enum_name,
            self.variant_name,
            self.deserializer,
        );

        Ok((value, variant_deserializer))
    }
}

fn deserialize_enum_error(enum_name: &str, variant_name: &str) -> ErrorKind {
    ErrorKind::DeserializeFailure(
        format!("enum variant {}::{}", enum_name, variant_name),
    )
}

mod variant_deserializer;

#[cfg(test)]
mod tests;
