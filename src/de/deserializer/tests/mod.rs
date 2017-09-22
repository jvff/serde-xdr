mod value;
mod visitor;

use std::io::Cursor;

use serde::Deserializer as SerdeDeserializer;

use super::Deserializer;

pub use self::value::Value;
pub use self::visitor::Visitor;

#[test]
fn deserialize_false() {
    let mut cursor = Cursor::new(vec![0x00, 0x00, 0x00, 0x00]);

    let result =
        Deserializer::new(&mut cursor).deserialize_bool(Visitor).unwrap();

    assert_eq!(cursor.position(), 4);
    assert_eq!(result, Value::Bool(false));
}

#[test]
fn deserialize_true() {
    let mut cursor = Cursor::new(vec![0x00, 0x00, 0x00, 0x01]);

    let result =
        Deserializer::new(&mut cursor).deserialize_bool(Visitor).unwrap();

    assert_eq!(cursor.position(), 4);
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn deserialize_i8() {
    let mut cursor = Cursor::new(vec![0xff, 0xff, 0xff, 0xfe]);

    let result =
        Deserializer::new(&mut cursor).deserialize_i8(Visitor).unwrap();

    assert_eq!(cursor.position(), 4);
    assert_eq!(result, Value::Integer8(-2));
}

#[test]
fn deserialize_i16() {
    let mut cursor = Cursor::new(vec![0xff, 0xff, 0xef, 0xfe]);

    let result =
        Deserializer::new(&mut cursor).deserialize_i16(Visitor).unwrap();

    let value_bits: u16 = 0xeffe;
    let expected_value = -((!value_bits) as i16 + 1);

    assert_eq!(cursor.position(), 4);
    assert_eq!(result, Value::Integer16(expected_value));
}

#[test]
fn deserialize_i32() {
    let mut cursor = Cursor::new(vec![0x80, 0x00, 0x00, 0x00]);

    let result =
        Deserializer::new(&mut cursor).deserialize_i32(Visitor).unwrap();

    assert_eq!(cursor.position(), 4);
    assert_eq!(result, Value::Integer32(-2_147_483_648));
}

#[test]
fn deserialize_i64() {
    let mut cursor = Cursor::new(
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_i64(Visitor).unwrap();

    assert_eq!(cursor.position(), 8);
    assert_eq!(result, Value::Integer64(-1));
}

#[test]
fn deserialize_u8() {
    let mut cursor = Cursor::new(vec![0x00, 0x00, 0x00, 0x0e]);

    let result =
        Deserializer::new(&mut cursor).deserialize_u8(Visitor).unwrap();

    assert_eq!(cursor.position(), 4);
    assert_eq!(result, Value::UnsignedInteger8(14));
}

#[test]
fn deserialize_u16() {
    let mut cursor = Cursor::new(vec![0x00, 0x00, 0x10, 0x0e]);

    let result =
        Deserializer::new(&mut cursor).deserialize_u16(Visitor).unwrap();

    assert_eq!(cursor.position(), 4);
    assert_eq!(result, Value::UnsignedInteger16(4110));
}

#[test]
fn deserialize_u32() {
    let mut cursor = Cursor::new(vec![0x80, 0x00, 0x10, 0x0e]);

    let result =
        Deserializer::new(&mut cursor).deserialize_u32(Visitor).unwrap();

    assert_eq!(cursor.position(), 4);
    assert_eq!(result, Value::UnsignedInteger32(0x8000_100e));
}

#[test]
fn deserialize_u64() {
    let mut cursor = Cursor::new(
        vec![0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_u64(Visitor).unwrap();

    assert_eq!(cursor.position(), 8);
    assert_eq!(result, Value::UnsignedInteger64(0x8000_0000_0000_0000));
}

#[test]
fn deserialize_f32() {
    let mut cursor = Cursor::new(vec![0xbf, 0x40, 0x00, 0x00]);

    let result =
        Deserializer::new(&mut cursor).deserialize_f32(Visitor).unwrap();

    assert_eq!(cursor.position(), 4);
    assert_eq!(result, Value::Float((-0.75).into()));
}

#[test]
fn deserialize_f64() {
    let mut cursor = Cursor::new(
        vec![0xbf, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_f64(Visitor).unwrap();

    assert_eq!(cursor.position(), 8);
    assert_eq!(result, Value::Double((-0.75).into()));
}

#[test]
fn deserialize_str_with_1_byte_padding() {
    let mut cursor = Cursor::new(
        vec![0x00, 0x00, 0x00, 0x03, 'H' as u8, 'i' as u8, '!' as u8, 0x00],
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_str(Visitor).unwrap();

    assert_eq!(cursor.position(), 8);
    assert_eq!(result, Value::String("Hi!".to_string()));
}

#[test]
fn deserialize_str_with_2_byte_padding() {
    let mut cursor = Cursor::new(
        vec![0x00, 0x00, 0x00, 0x02, 'H' as u8, 'i' as u8, 0x00, 0x00],
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_str(Visitor).unwrap();

    assert_eq!(cursor.position(), 8);
    assert_eq!(result, Value::String("Hi".to_string()));
}

#[test]
fn deserialize_str_with_3_byte_padding() {
    let mut cursor = Cursor::new(
        vec![
            0x00, 0x00, 0x00, 0x05,
            'H' as u8, 'e' as u8, 'l' as u8, 'l' as u8,
            'o' as u8, 0x00, 0x00, 0x00,
        ],
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_str(Visitor).unwrap();

    assert_eq!(cursor.position(), 12);
    assert_eq!(result, Value::String("Hello".to_string()));
}

#[test]
fn deserialize_str_without_padding() {
    let mut cursor = Cursor::new(
        vec![
            0x00, 0x00, 0x00, 0x04,
            'H' as u8, 'e' as u8, 'y' as u8, '!' as u8,
        ],
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_str(Visitor).unwrap();

    assert_eq!(cursor.position(), 8);
    assert_eq!(result, Value::String("Hey!".to_string()));
}

#[test]
fn deserialize_opaque_without_padding() {
    let mut cursor = Cursor::new(
        vec![
            0x00, 0x00, 0x00, 0x08,
            0x01, 0x02, 0x03, 0x04, 0xff, 0xfe, 0xfd, 0xfc,
        ],
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_bytes(Visitor).unwrap();

    let expected_bytes = vec![0x01, 0x02, 0x03, 0x04, 0xff, 0xfe, 0xfd, 0xfc];

    assert_eq!(cursor.position(), 12);
    assert_eq!(result, Value::Bytes(expected_bytes));
}

#[test]
fn deserialize_opaque_with_1_byte_padding() {
    let mut cursor = Cursor::new(
        vec![
            0x00, 0x00, 0x00, 0x07,
            0x01, 0x02, 0x03, 0x04, 0xff, 0xfe, 0xfd, 0x00,
        ],
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_bytes(Visitor).unwrap();

    let expected_bytes = vec![0x01, 0x02, 0x03, 0x04, 0xff, 0xfe, 0xfd];

    assert_eq!(cursor.position(), 12);
    assert_eq!(result, Value::Bytes(expected_bytes));
}

#[test]
fn deserialize_opaque_with_2_byte_padding() {
    let mut cursor = Cursor::new(
        vec![
            0x00, 0x00, 0x00, 0x06,
            0x01, 0x02, 0x03, 0x04, 0xff, 0xfe, 0x00, 0x00,
        ],
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_bytes(Visitor).unwrap();

    let expected_bytes = vec![0x01, 0x02, 0x03, 0x04, 0xff, 0xfe];

    assert_eq!(cursor.position(), 12);
    assert_eq!(result, Value::Bytes(expected_bytes));
}

#[test]
fn deserialize_opaque_with_3_byte_padding() {
    let mut cursor = Cursor::new(
        vec![
            0x00, 0x00, 0x00, 0x05,
            0x01, 0x02, 0x03, 0x04, 0xff, 0x00, 0x00, 0x00,
        ],
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_bytes(Visitor).unwrap();

    let expected_bytes = vec![0x01, 0x02, 0x03, 0x04, 0xff];

    assert_eq!(cursor.position(), 12);
    assert_eq!(result, Value::Bytes(expected_bytes));
}

#[test]
fn deserialize_none() {
    let mut cursor = Cursor::new(vec![0x00, 0x00, 0x00, 0x00]);

    let result =
        Deserializer::new(&mut cursor).deserialize_option(Visitor).unwrap();

    assert_eq!(cursor.position(), 4);
    assert_eq!(result, Value::None);
}

#[test]
fn deserialize_some() {
    let mut cursor = Cursor::new(
        vec![0x00, 0x00, 0x00, 0x01, 0xab, 0xcd, 0xef, 0x98],
    );

    let result =
        Deserializer::new(&mut cursor).deserialize_option(Visitor).unwrap();

    assert_eq!(cursor.position(), 8);
    assert_eq!(result, Value::UnsignedInteger32(0xabcdef98));
}

#[test]
fn deserialize_unit() {
    let mut cursor = Cursor::new(vec![]);

    let result =
        Deserializer::new(&mut cursor).deserialize_unit(Visitor).unwrap();

    assert_eq!(cursor.position(), 0);
    assert_eq!(result, Value::Unit);
}

#[test]
fn deserialize_newtype_struct() {
    let mut cursor = Cursor::new(vec![0xff, 0xff, 0xff, 0xf0]);

    let result = Deserializer::new(&mut cursor)
        .deserialize_newtype_struct("wrapper", Visitor)
        .unwrap();

    assert_eq!(cursor.position(), 4);
    assert_eq!(result, Value::Integer32(-16));
}
