use std::io::Cursor;

use serde::Deserializer as SerdeDeserializer;

use super::super::Deserializer;
use super::super::tests::{Value, Visitor};

#[test]
fn deserialize_first_enum_variant() {
    static VARIANT_NAMES: [&str; 3] = ["bool", "string", "integer"];

    let mut cursor = Cursor::new(vec![
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
    ]);

    let result = Deserializer::new(&mut cursor)
        .deserialize_enum("enum", &VARIANT_NAMES, Visitor)
        .unwrap();

    let expected_result = Value::Enum(
        0,
        Box::new(Value::Bool(true)),
    );

    assert_eq!(cursor.position(), 8);
    assert_eq!(result, expected_result);
}

#[test]
fn deserialize_second_enum_variant() {
    static VARIANT_NAMES: [&str; 3] = ["bool", "string", "integer"];

    let mut cursor = Cursor::new(vec![
        0x00, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x03,
        'H' as u8, 'i' as u8, '!' as u8, 0x00,
    ]);

    let result = Deserializer::new(&mut cursor)
        .deserialize_enum("enum", &VARIANT_NAMES, Visitor)
        .unwrap();

    let expected_result = Value::Enum(
        1,
        Box::new(Value::String("Hi!".to_string())),
    );

    assert_eq!(cursor.position(), 12);
    assert_eq!(result, expected_result);
}

#[test]
fn deserialize_third_enum_variant() {
    static VARIANT_NAMES: [&str; 3] = ["bool", "string", "integer"];

    let mut cursor = Cursor::new(vec![
        0x00, 0x00, 0x00, 0x02,
        0xff, 0xff, 0xff, 0xfd,
    ]);

    let result = Deserializer::new(&mut cursor)
        .deserialize_enum("enum", &VARIANT_NAMES, Visitor)
        .unwrap();

    let expected_result = Value::Enum(
        2,
        Box::new(Value::Integer32(-3)),
    );

    assert_eq!(cursor.position(), 8);
    assert_eq!(result, expected_result);
}
