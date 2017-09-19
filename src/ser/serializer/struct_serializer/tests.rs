use serde::ser::Serializer as SerdeSerializer;
use serde::ser::SerializeStruct;

use super::*;
use super::super::tests::*;

#[test]
fn serialize_struct() {
    let mut buffer = Vec::new();
    let number: i32 = -20;
    let string = "hello";

    {
        let mut struct_serializer =
            Serializer::new(&mut buffer).serialize_struct("struct", 2).unwrap();

        struct_serializer.serialize_field("number", &number).unwrap();
        struct_serializer.serialize_field("string", string).unwrap();
        struct_serializer.end().unwrap();
    }

    let mut expected_bytes = bytes_of(number as u32);

    expected_bytes.append(&mut bytes_of_str(string, 2));

    assert_eq!(buffer, expected_bytes);
}
