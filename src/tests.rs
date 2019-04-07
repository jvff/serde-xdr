use std::io::Cursor;

use serde_bytes;

use super::{from_bytes, from_reader, to_bytes, to_writer};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
enum FileType {
    Text,
    Data(String),
    Exec(String),
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct File {
    filename: String,
    filetype: FileType,
    owner: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

#[test]
fn serialization_functions_are_equivalent() {
    let file_contents: Vec<u8> = "(quit)".as_bytes().into();

    let file = File {
        filename: "sillyprog".to_string(),
        filetype: FileType::Exec("lisp".to_string()),
        owner: "john".to_string(),
        data: file_contents,
    };

    let serialized_to_bytes = to_bytes(&file).unwrap();
    let mut serialized_to_writer = Vec::new();

    to_writer(&mut serialized_to_writer, &file).unwrap();

    assert_eq!(serialized_to_bytes, serialized_to_writer);
}

#[test]
fn serialized_bytes() {
    let file_contents: Vec<u8> = "(quit)".as_bytes().into();

    let file = File {
        filename: "sillyprog".to_string(),
        filetype: FileType::Exec("lisp".to_string()),
        owner: "john".to_string(),
        data: file_contents,
    };

    let bytes = to_bytes(&file).unwrap();

    let expected_bytes = vec![
        0x00, 0x00, 0x00, 0x09, 's' as u8, 'i' as u8, 'l' as u8, 'l' as u8,
        'y' as u8, 'p' as u8, 'r' as u8, 'o' as u8, 'g' as u8, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 'l' as u8,
        'i' as u8, 's' as u8, 'p' as u8, 0x00, 0x00, 0x00, 0x04, 'j' as u8,
        'o' as u8, 'h' as u8, 'n' as u8, 0x00, 0x00, 0x00, 0x06, '(' as u8,
        'q' as u8, 'u' as u8, 'i' as u8, 't' as u8, ')' as u8, 0x00, 0x00,
    ];

    assert_eq!(bytes, expected_bytes);
}

#[test]
fn serialize_deserialize() {
    let file_contents: Vec<u8> = "(quit)".as_bytes().into();

    let initial_file = File {
        filename: "sillyprog".to_string(),
        filetype: FileType::Exec("lisp".to_string()),
        owner: "john".to_string(),
        data: file_contents,
    };

    let bytes = to_bytes(&initial_file).unwrap();
    let length = bytes.len() as u64;
    let mut cursor = Cursor::new(bytes);

    let recovered_file = from_reader(&mut cursor).unwrap();

    assert_eq!(initial_file, recovered_file);
    assert_eq!(cursor.position(), length);
}

#[test]
fn deserialization_functions_are_equivalent() {
    let source_bytes = vec![
        0x00, 0x00, 0x00, 0x09, 's' as u8, 'i' as u8, 'l' as u8, 'l' as u8,
        'y' as u8, 'p' as u8, 'r' as u8, 'o' as u8, 'g' as u8, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 'l' as u8,
        'i' as u8, 's' as u8, 'p' as u8, 0x00, 0x00, 0x00, 0x04, 'j' as u8,
        'o' as u8, 'h' as u8, 'n' as u8, 0x00, 0x00, 0x00, 0x06, '(' as u8,
        'q' as u8, 'u' as u8, 'i' as u8, 't' as u8, ')' as u8, 0x00, 0x00,
    ];

    let deserialized_from_bytes: File = from_bytes(&source_bytes).unwrap();

    let mut reader = Cursor::new(source_bytes);
    let deserialized_from_reader: File = from_reader(&mut reader).unwrap();

    assert_eq!(deserialized_from_bytes, deserialized_from_reader);
}
