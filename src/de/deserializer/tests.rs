use std::fmt;
use std::fmt::Formatter;
use std::io::Cursor;

use serde::de;
use serde::Deserializer as SerdeDeserializer;

use super::Deserializer;

#[derive(Debug, Eq, PartialEq)]
enum Value {
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    UnsignedInteger8(u8),
    UnsignedInteger16(u16),
    UnsignedInteger32(u32),
    UnsignedInteger64(u64),
}

struct Visitor;

macro_rules! visit_methods {
    ( $( $name:ident ( $base_type:ty ) -> $value_type:ident ),* $(,)* ) => {
        $(
            fn $name<E>(self, value: $base_type) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::$value_type(value))
            }
        )*
    }
}

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "unknown")
    }

    visit_methods! {
        visit_i8(i8) -> Integer8,
        visit_i16(i16) -> Integer16,
        visit_i32(i32) -> Integer32,
        visit_i64(i64) -> Integer64,
        visit_u8(u8) -> UnsignedInteger8,
        visit_u16(u16) -> UnsignedInteger16,
        visit_u32(u32) -> UnsignedInteger32,
        visit_u64(u64) -> UnsignedInteger64,
    }
}

#[test]
fn deserialize_i8() {
    let mut cursor = Cursor::new(vec![0xff, 0xff, 0xff, 0xfe]);

    let result =
        Deserializer::new(&mut cursor).deserialize_i8(Visitor).unwrap();

    assert_eq!(result, Value::Integer8(-2));
}

#[test]
fn deserialize_i16() {
    let mut cursor = Cursor::new(vec![0xff, 0xff, 0xef, 0xfe]);

    let result =
        Deserializer::new(&mut cursor).deserialize_i16(Visitor).unwrap();

    let value_bits: u16 = 0xeffe;
    let expected_value = -((!value_bits) as i16 + 1);

    assert_eq!(result, Value::Integer16(expected_value));
}

#[test]
fn deserialize_i32() {
    let mut cursor = Cursor::new(vec![0x80, 0x00, 0x00, 0x00]);

    let result =
        Deserializer::new(&mut cursor).deserialize_i32(Visitor).unwrap();

    assert_eq!(result, Value::Integer32(-2_147_483_648));
}

#[test]
fn deserialize_i64() {
    let mut cursor = Cursor::new(
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_i64(Visitor).unwrap();

    assert_eq!(result, Value::Integer64(-1));
}

#[test]
fn deserialize_u8() {
    let mut cursor = Cursor::new(vec![0x00, 0x00, 0x00, 0x0e]);

    let result =
        Deserializer::new(&mut cursor).deserialize_u8(Visitor).unwrap();

    assert_eq!(result, Value::UnsignedInteger8(14));
}

#[test]
fn deserialize_u16() {
    let mut cursor = Cursor::new(vec![0x00, 0x00, 0x10, 0x0e]);

    let result =
        Deserializer::new(&mut cursor).deserialize_u16(Visitor).unwrap();

    assert_eq!(result, Value::UnsignedInteger16(4110));
}

#[test]
fn deserialize_u32() {
    let mut cursor = Cursor::new(vec![0x80, 0x00, 0x10, 0x0e]);

    let result =
        Deserializer::new(&mut cursor).deserialize_u32(Visitor).unwrap();

    assert_eq!(result, Value::UnsignedInteger32(0x8000_100e));
}

#[test]
fn deserialize_u64() {
    let mut cursor = Cursor::new(
        vec![0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_u64(Visitor).unwrap();

    assert_eq!(result, Value::UnsignedInteger64(0x8000_0000_0000_0000));
}
