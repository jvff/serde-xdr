use serde::ser::Serializer as SerdeSerializer;

use super::Serializer;

fn bytes_of(mut value: u32) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4);

    for _ in 0..4 {
        let byte = (value >> 24) as u8;

        bytes.push(byte);

        value <<= 8;
    }

    bytes
}

#[test]
fn serialize_i8() {
    let mut buffer = Vec::new();
    let value = -120;

    Serializer::new(&mut buffer).serialize_i8(value).unwrap();

    assert_eq!(buffer, bytes_of(value as u32));
}

#[test]
fn serialize_i16() {
    let mut buffer = Vec::new();
    let value = -15000;

    Serializer::new(&mut buffer).serialize_i16(value).unwrap();

    assert_eq!(buffer, bytes_of(value as u32));
}

#[test]
fn serialize_i32() {
    let mut buffer = Vec::new();
    let value = -1785082;

    Serializer::new(&mut buffer).serialize_i32(value).unwrap();

    assert_eq!(buffer, bytes_of(value as u32));
}

#[test]
fn serialize_u8() {
    let mut buffer = Vec::new();
    let value = 249;

    Serializer::new(&mut buffer).serialize_u8(value).unwrap();

    assert_eq!(buffer, bytes_of(value as u32));
}

#[test]
fn serialize_u16() {
    let mut buffer = Vec::new();
    let value = 65412;

    Serializer::new(&mut buffer).serialize_u16(value).unwrap();

    assert_eq!(buffer, bytes_of(value as u32));
}

#[test]
fn serialize_u32() {
    let mut buffer = Vec::new();
    let value = 4230834;

    Serializer::new(&mut buffer).serialize_u32(value).unwrap();

    assert_eq!(buffer, bytes_of(value));
}

#[test]
fn serialize_enum() {
    let mut buffer = Vec::new();
    let variant_index = 300;

    Serializer::new(&mut buffer)
        .serialize_unit_variant("name", variant_index, "4")
        .unwrap();

    assert_eq!(buffer, bytes_of(variant_index));
}
