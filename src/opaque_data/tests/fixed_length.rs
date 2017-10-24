use super::super::fixed_length;
use super::super::super::to_bytes;

#[derive(Serialize)]
struct FourBytes {
    #[serde(serialize_with = "fixed_length::serialize")]
    data: [u8; 4],
}

#[derive(Serialize)]
struct FiveBytes {
    #[serde(serialize_with = "fixed_length::serialize")]
    data: [u8; 5],
}

#[derive(Serialize)]
struct SixBytes {
    #[serde(serialize_with = "fixed_length::serialize")]
    data: [u8; 6],
}

#[derive(Serialize)]
struct SevenBytes {
    #[serde(serialize_with = "fixed_length::serialize")]
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
