use serde::ser::Serializer as SerdeSerializer;

use super::super::tests::*;
use super::super::super::Serializer;

#[test]
fn serialize_sequence() {
    let mut buffer = Vec::new();

    let length = 3;
    let first_element: i32 = -20;
    let second_element = "hello";
    let third_element: i32 = -45;

    {
        use serde::ser::SerializeSeq;

        let mut sequence_serializer =
            Serializer::new(&mut buffer).serialize_seq(Some(length)).unwrap();

        sequence_serializer.serialize_element(&first_element).unwrap();
        sequence_serializer.serialize_element(second_element).unwrap();
        sequence_serializer.serialize_element(&third_element).unwrap();
        sequence_serializer.end().unwrap();
    }

    let mut expected_bytes = bytes_of(length as u32);

    expected_bytes.append(&mut bytes_of(first_element as u32));
    expected_bytes.append(&mut bytes_of_str(second_element, 3));
    expected_bytes.append(&mut bytes_of(third_element as u32));

    assert_eq!(buffer, expected_bytes);
}

#[test]
fn serialize_tuple() {
    let mut buffer = Vec::new();

    let length = 3;
    let first_element: i32 = -20;
    let second_element = "hello";
    let third_element: i32 = -45;

    {
        use serde::ser::SerializeTuple;

        let mut sequence_serializer =
            Serializer::new(&mut buffer).serialize_tuple(length).unwrap();

        sequence_serializer.serialize_element(&first_element).unwrap();
        sequence_serializer.serialize_element(second_element).unwrap();
        sequence_serializer.serialize_element(&third_element).unwrap();
        sequence_serializer.end().unwrap();
    }

    let mut expected_bytes = Vec::new();

    expected_bytes.append(&mut bytes_of(first_element as u32));
    expected_bytes.append(&mut bytes_of_str(second_element, 3));
    expected_bytes.append(&mut bytes_of(third_element as u32));

    assert_eq!(buffer, expected_bytes);
}

