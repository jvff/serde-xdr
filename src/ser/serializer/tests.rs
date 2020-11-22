use {super::Serializer, serde::ser::Serializer as _, std::mem};

pub fn bytes_of(mut value: u32) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4);

    for _ in 0..4 {
        let byte = (value >> 24) as u8;

        bytes.push(byte);

        value <<= 8;
    }

    bytes
}

pub fn bytes_of_hyper(mut value: u64) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(8);

    for _ in 0..8 {
        let byte = (value >> 56) as u8;

        bytes.push(byte);

        value <<= 8;
    }

    bytes
}

pub fn bytes_of_f32(value: f32) -> Vec<u8> {
    let bits: u32;

    unsafe {
        bits = mem::transmute::<f32, u32>(value);
    }

    bytes_of(bits)
}

pub fn bytes_of_f64(value: f64) -> Vec<u8> {
    let bits: u64;

    unsafe {
        bits = mem::transmute::<f64, u64>(value);
    }

    bytes_of_hyper(bits)
}

pub fn bytes_of_opaque(opaque: &[u8], padding_length: usize) -> Vec<u8> {
    let padded_length = 4 + opaque.len() + padding_length;

    let mut bytes = bytes_of(opaque.len() as u32);

    bytes.append(&mut opaque.iter().cloned().collect());
    bytes.resize(padded_length, 0);

    bytes
}

pub fn bytes_of_str(string: &str, padding_length: usize) -> Vec<u8> {
    bytes_of_opaque(string.as_bytes(), padding_length)
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
fn serialize_char() {
    let mut buffer = Vec::new();
    let value = '\u{2764}';

    Serializer::new(&mut buffer).serialize_char(value).unwrap();

    assert_eq!(buffer, bytes_of(value as u32));
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

#[test]
fn serialize_true() {
    let mut buffer = Vec::new();

    Serializer::new(&mut buffer).serialize_bool(true).unwrap();

    assert_eq!(buffer, bytes_of(1));
}

#[test]
fn serialize_false() {
    let mut buffer = Vec::new();

    Serializer::new(&mut buffer).serialize_bool(false).unwrap();

    assert_eq!(buffer, bytes_of(0));
}

#[test]
fn serialize_i64() {
    let mut buffer = Vec::new();
    let value = 6_980_010_427_672;

    Serializer::new(&mut buffer).serialize_i64(value).unwrap();

    assert_eq!(buffer, bytes_of_hyper(value as u64));
}

#[test]
fn serialize_u64() {
    let mut buffer = Vec::new();
    let value = 27_422_481_429;

    Serializer::new(&mut buffer).serialize_u64(value).unwrap();

    assert_eq!(buffer, bytes_of_hyper(value));
}

#[test]
fn serialize_f32() {
    let mut buffer = Vec::new();
    let value = 0.002708;

    Serializer::new(&mut buffer).serialize_f32(value).unwrap();

    assert_eq!(buffer, bytes_of_f32(value));
}

#[test]
fn serialize_f64() {
    let mut buffer = Vec::new();
    let value = 0.0000000013073298;

    Serializer::new(&mut buffer).serialize_f64(value).unwrap();

    assert_eq!(buffer, bytes_of_f64(value));
}

#[test]
fn serialize_opaque_without_padding() {
    let mut buffer = Vec::new();
    let value = vec![1, 2, 3, 4, 8, 7, 6, 5];

    Serializer::new(&mut buffer)
        .serialize_bytes(&value)
        .unwrap();

    assert_eq!(buffer, bytes_of_opaque(&value, 0));
}

#[test]
fn serialize_opaque_with_1_byte_padding() {
    let mut buffer = Vec::new();
    let value = vec![1, 2, 3, 4, 8, 7, 6, 5, 9, 10, 11];

    Serializer::new(&mut buffer)
        .serialize_bytes(&value)
        .unwrap();

    assert_eq!(buffer, bytes_of_opaque(&value, 1));
}

#[test]
fn serialize_opaque_with_2_byte_padding() {
    let mut buffer = Vec::new();
    let value = vec![1, 2, 3, 4, 8, 7, 6, 5, 9, 10];

    Serializer::new(&mut buffer)
        .serialize_bytes(&value)
        .unwrap();

    assert_eq!(buffer, bytes_of_opaque(&value, 2));
}

#[test]
fn serialize_opaque_with_3_byte_padding() {
    let mut buffer = Vec::new();
    let value = vec![1, 2, 3, 4, 8, 7, 6, 5, 9];

    Serializer::new(&mut buffer)
        .serialize_bytes(&value)
        .unwrap();

    assert_eq!(buffer, bytes_of_opaque(&value, 3));
}

#[test]
fn serialize_string_with_1_byte_padding() {
    let mut buffer = Vec::new();
    let value = "a simple string";

    Serializer::new(&mut buffer).serialize_str(&value).unwrap();

    assert_eq!(buffer, bytes_of_str(value, 1));
}

#[test]
fn serialize_string_with_2_byte_padding() {
    let mut buffer = Vec::new();
    let value = "a simple string...";

    Serializer::new(&mut buffer).serialize_str(&value).unwrap();

    assert_eq!(buffer, bytes_of_str(value, 2));
}

#[test]
fn serialize_string_with_3_byte_padding() {
    let mut buffer = Vec::new();
    let value = "a string.";

    Serializer::new(&mut buffer).serialize_str(&value).unwrap();

    assert_eq!(buffer, bytes_of_str(value, 3));
}

#[test]
fn serialize_string_without_padding() {
    let mut buffer = Vec::new();
    let value = "a simple string!";

    Serializer::new(&mut buffer).serialize_str(&value).unwrap();

    assert_eq!(buffer, bytes_of_str(value, 0));
}

#[test]
fn serialize_void() {
    let mut buffer = Vec::new();

    Serializer::new(&mut buffer).serialize_unit().unwrap();

    assert_eq!(buffer.len(), 0);
}

#[test]
fn serialize_unit_struct() {
    let mut buffer = Vec::new();

    Serializer::new(&mut buffer)
        .serialize_unit_struct("name")
        .unwrap();

    assert_eq!(buffer.len(), 0);
}

#[test]
fn serialize_wrapped_type() {
    let mut buffer = Vec::new();
    let value = 4230;

    Serializer::new(&mut buffer)
        .serialize_newtype_struct("wrapped_u32", &value)
        .unwrap();

    assert_eq!(buffer, bytes_of(value));
}

#[test]
fn serialize_union() {
    let mut buffer = Vec::new();
    let value = "a simple string";
    let variant_index = 5;

    Serializer::new(&mut buffer)
        .serialize_newtype_variant("union", variant_index, "variant", value)
        .unwrap();

    let mut expected_bytes = bytes_of(variant_index);

    expected_bytes.append(&mut bytes_of_str(value, 1));

    assert_eq!(buffer, expected_bytes);
}

#[test]
fn serialize_none() {
    let mut buffer = Vec::new();

    Serializer::new(&mut buffer).serialize_none().unwrap();

    assert_eq!(buffer, bytes_of(0));
}

#[test]
fn serialize_some() {
    let mut buffer = Vec::new();
    let value: u32 = 1000;

    Serializer::new(&mut buffer).serialize_some(&value).unwrap();

    let mut expected_bytes = bytes_of(1);

    expected_bytes.append(&mut bytes_of(value));

    assert_eq!(buffer, expected_bytes);
}

#[test]
fn serialize_tuple_variant() {
    use serde::ser::SerializeTuple as _;

    let mut buffer = Vec::new();

    let variant_index = 5;
    let tuple_elements = 2;
    let first_element = false;
    let second_element = -3i16;

    let mut serializer = Serializer::new(&mut buffer)
        .serialize_tuple_variant(
            "MyEnum",
            variant_index,
            "SixthVariant",
            tuple_elements,
        )
        .unwrap();

    serializer.serialize_element(&first_element).unwrap();
    serializer.serialize_element(&second_element).unwrap();
    serializer.end().unwrap();

    let mut expected_bytes = bytes_of(variant_index);

    expected_bytes.append(&mut bytes_of(0));
    expected_bytes.append(&mut bytes_of(second_element as u32));

    assert_eq!(buffer, expected_bytes);
}

#[test]
fn serialize_struct_variant() {
    use serde::ser::SerializeStruct as _;

    let mut buffer = Vec::new();

    let variant_index = 3;
    let struct_fields = 2;
    let first_field_value = false;
    let second_field_value = -3i16;

    let mut serializer = Serializer::new(&mut buffer)
        .serialize_struct_variant(
            "MyEnum",
            variant_index,
            "FourthVariant",
            struct_fields,
        )
        .unwrap();

    serializer
        .serialize_field("first_field", &first_field_value)
        .unwrap();
    serializer
        .serialize_field("second_field", &second_field_value)
        .unwrap();
    serializer.end().unwrap();

    let mut expected_bytes = bytes_of(variant_index);

    expected_bytes.append(&mut bytes_of(0));
    expected_bytes.append(&mut bytes_of(second_field_value as u32));

    assert_eq!(buffer, expected_bytes);
}
