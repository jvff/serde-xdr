use std::io::Cursor;

use super::OpaqueData;
use super::super::{from_reader, to_bytes};

#[test]
fn from_vec() {
    let vec = vec![0, 1, 2, 3, 4];

    let opaque_data = OpaqueData {
        data: vec.clone(),
    };

    assert_eq!(OpaqueData::from(vec), opaque_data);
}

#[test]
fn to_vec() {
    let vec = vec![0, 1, 2, 3, 4];

    let opaque_data = OpaqueData {
        data: vec.clone(),
    };

    assert_eq!(Into::<Vec<u8>>::into(opaque_data), vec);
}

#[test]
fn deref_to_vec() {
    let vec = vec![0, 1, 2, 3, 4];

    let opaque_data = OpaqueData {
        data: vec.clone(),
    };

    assert_eq!(opaque_data.len(), vec.len());
}

#[test]
fn deref_mut_to_vec() {
    let mut vec = vec![0, 1, 2, 3, 4];

    let mut opaque_data = OpaqueData {
        data: vec.clone(),
    };

    vec.push(5);
    opaque_data.push(5);

    assert_eq!(opaque_data.data, vec);
}

#[test]
fn serialize() {
    let opaque_data = OpaqueData {
        data: vec![0, 1, 2, 3, 4],
    };

    let bytes = to_bytes(&opaque_data).unwrap();

    let expected_bytes = vec![
        0x00, 0x00, 0x00, 0x05,
        0x00, 0x01, 0x02, 0x03,
        0x04, 0x00, 0x00, 0x00,
    ];

    assert_eq!(bytes, expected_bytes);
}

#[test]
fn deserialize() {
    let bytes = vec![
        0x00, 0x00, 0x00, 0x05,
        0x00, 0x01, 0x02, 0x03,
        0x04, 0x00, 0x00, 0x00,
    ];
    let mut cursor = Cursor::new(bytes);

    let opaque_data: OpaqueData = from_reader(&mut cursor).unwrap();

    let expected_opaque_data = OpaqueData {
        data: vec![0, 1, 2, 3, 4],
    };

    assert_eq!(opaque_data, expected_opaque_data);
}
