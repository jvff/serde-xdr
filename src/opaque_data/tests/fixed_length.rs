use std::io::Cursor;

use super::super::super::{from_reader, to_bytes};
use super::super::fixed_length;

#[derive(Deserialize, Serialize)]
struct FourBytes {
    #[serde(with = "fixed_length")]
    data: [u8; 4],
}

#[derive(Deserialize, Serialize)]
struct FiveBytes {
    #[serde(with = "fixed_length")]
    data: [u8; 5],
}

#[derive(Deserialize, Serialize)]
struct SixBytes {
    #[serde(with = "fixed_length")]
    data: [u8; 6],
}

#[derive(Deserialize, Serialize)]
struct SevenBytes {
    #[serde(with = "fixed_length")]
    data: [u8; 7],
}

#[test]
fn serialize_bytes_without_padding() {
    let data = [0, 1, 2, 4];
    let bytes = FourBytes { data };

    let serialized_bytes = to_bytes(&bytes).unwrap();

    assert_eq!(serialized_bytes, vec![0, 1, 2, 4]);
}

#[test]
fn serialize_bytes_with_3_byte_padding() {
    let data = [0, 1, 2, 4, 8];
    let bytes = FiveBytes { data };

    let serialized_bytes = to_bytes(&bytes).unwrap();

    assert_eq!(serialized_bytes, vec![0, 1, 2, 4, 8, 0, 0, 0]);
}

#[test]
fn serialize_bytes_with_2_byte_padding() {
    let data = [0, 1, 2, 4, 8, 16];
    let bytes = SixBytes { data };

    let serialized_bytes = to_bytes(&bytes).unwrap();

    assert_eq!(serialized_bytes, vec![0, 1, 2, 4, 8, 16, 0, 0]);
}

#[test]
fn serialize_bytes_with_1_byte_padding() {
    let data = [0, 1, 2, 4, 8, 16, 32];
    let bytes = SevenBytes { data };

    let serialized_bytes = to_bytes(&bytes).unwrap();

    assert_eq!(serialized_bytes, vec![0, 1, 2, 4, 8, 16, 32, 0]);
}

#[test]
fn deserialize_bytes_without_padding() {
    let serialized_bytes = [0, 1, 2, 4];
    let mut cursor = Cursor::new(serialized_bytes);

    let bytes: FourBytes = from_reader(&mut cursor).unwrap();

    assert_eq!(bytes.data, [0, 1, 2, 4]);
}

#[test]
fn deserialize_bytes_with_3_byte_padding() {
    let serialized_bytes = [0, 1, 2, 4, 8, 0, 0, 0];
    let mut cursor = Cursor::new(serialized_bytes);

    let bytes: FiveBytes = from_reader(&mut cursor).unwrap();

    assert_eq!(bytes.data, [0, 1, 2, 4, 8]);
}

#[test]
fn deserialize_bytes_with_2_byte_padding() {
    let serialized_bytes = [0, 1, 2, 4, 8, 16, 0, 0];
    let mut cursor = Cursor::new(serialized_bytes);

    let bytes: SixBytes = from_reader(&mut cursor).unwrap();

    assert_eq!(bytes.data, [0, 1, 2, 4, 8, 16]);
}

#[test]
fn deserialize_bytes_with_1_byte_padding() {
    let serialized_bytes = [0, 1, 2, 4, 8, 16, 32, 0];
    let mut cursor = Cursor::new(serialized_bytes);

    let bytes: SevenBytes = from_reader(&mut cursor).unwrap();

    assert_eq!(bytes.data, [0, 1, 2, 4, 8, 16, 32]);
}
