use serde::ser::Serializer as SerdeSerializer;

use super::Serializer;

#[test]
fn serialize_i8() {
    let mut buffer = Vec::new();
    let value = -120;

    Serializer::new(&mut buffer).serialize_i8(value).unwrap();

    let expected_byte = 0xff - (-value as u8) + 1;

    assert_eq!(buffer, vec![0xff, 0xff, 0xff, expected_byte]);
}

#[test]
fn serialize_i16() {
    let mut buffer = Vec::new();
    let value = -15000;

    Serializer::new(&mut buffer).serialize_i16(value).unwrap();

    let expected_value = 0xffff - (-value as u16) + 1;
    let expected_msb = (expected_value >> 8) as u8;
    let expected_lsb = (expected_value & 0xff) as u8;

    assert_eq!(buffer, vec![0xff, 0xff, expected_msb, expected_lsb]);
}

#[test]
fn serialize_i32() {
    let mut buffer = Vec::new();
    let value = -1785082;

    Serializer::new(&mut buffer).serialize_i32(value).unwrap();

    let mut expected_value = 0xffffffff - (-value as u32) + 1;
    let mut expected_bytes = Vec::with_capacity(4);

    for _ in 0..4 {
        let byte = (expected_value >> 24) as u8;

        expected_bytes.push(byte);

        expected_value <<= 8;
    }

    assert_eq!(buffer, expected_bytes);
}

#[test]
fn serialize_u8() {
    let mut buffer = Vec::new();
    let value = 249;

    Serializer::new(&mut buffer).serialize_u8(value).unwrap();

    assert_eq!(buffer, vec![0, 0, 0, value]);
}

#[test]
fn serialize_u16() {
    let mut buffer = Vec::new();
    let value = 65412;

    Serializer::new(&mut buffer).serialize_u16(value).unwrap();

    let expected_msb = (value >> 8) as u8;
    let expected_lsb = (value & 0xff) as u8;

    assert_eq!(buffer, vec![0, 0, expected_msb, expected_lsb]);
}

#[test]
fn serialize_u32() {
    let mut buffer = Vec::new();
    let value = 4230834;

    Serializer::new(&mut buffer).serialize_u32(value).unwrap();

    let mut expected_value = value;
    let mut expected_bytes = Vec::with_capacity(4);

    for _ in 0..4 {
        let byte = (expected_value >> 24) as u8;

        expected_bytes.push(byte);

        expected_value <<= 8;
    }

    assert_eq!(buffer, expected_bytes);
}
