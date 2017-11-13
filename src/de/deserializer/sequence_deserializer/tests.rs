use std::io::Cursor;

use serde::Deserializer as SerdeDeserializer;

use super::super::Deserializer;
use super::super::tests::{Value, Visitor};

#[test]
fn deserialize_sequence() {
    let mut cursor = Cursor::new(vec![
        0x00,
        0x00,
        0x00,
        0x03,
        0x00,
        0x00,
        0x00,
        0x01,
        0x00,
        0x00,
        0x00,
        0x04,
        'H' as u8,
        'e' as u8,
        'y' as u8,
        '!' as u8,
        0xff,
        0xff,
        0xff,
        0xfb,
    ]);

    let result = Deserializer::new(&mut cursor)
        .deserialize_seq(Visitor)
        .unwrap();

    let expected_result = Value::Sequence(vec![
        Value::Bool(true),
        Value::String("Hey!".to_string()),
        Value::Integer32(-5),
    ]);

    assert_eq!(cursor.position(), 20);
    assert_eq!(result, expected_result);
}

#[test]
fn deserialize_tuple() {
    let mut cursor = Cursor::new(vec![
        0x00,
        0x00,
        0x00,
        0x01,
        0x00,
        0x00,
        0x00,
        0x04,
        'H' as u8,
        'e' as u8,
        'y' as u8,
        '!' as u8,
        0xff,
        0xff,
        0xff,
        0xfb,
    ]);

    let result = Deserializer::new(&mut cursor)
        .deserialize_tuple(3, Visitor)
        .unwrap();

    let expected_result = Value::Sequence(vec![
        Value::Bool(true),
        Value::String("Hey!".to_string()),
        Value::Integer32(-5),
    ]);

    assert_eq!(cursor.position(), 16);
    assert_eq!(result, expected_result);
}
