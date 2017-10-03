use std::io::Cursor;

use super::{from_reader, to_bytes};
use super::VariableLengthOpaqueData;

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
    data: VariableLengthOpaqueData,
}

#[test]
fn serialized_bytes() {
    let file_contents: Vec<u8> = "(quit)".as_bytes().into();

    let file = File {
        filename: "sillyprog".to_string(),
        filetype: FileType::Exec("lisp".to_string()),
        owner: "john".to_string(),
        data: file_contents.into(),
    };

    let bytes = to_bytes(&file).unwrap();

    let expected_bytes = vec![
        0x00, 0x00, 0x00, 0x09,
        's' as u8, 'i' as u8, 'l' as u8, 'l' as u8,
        'y' as u8, 'p' as u8, 'r' as u8, 'o' as u8,
        'g' as u8, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x04,
        'l' as u8, 'i' as u8, 's' as u8, 'p' as u8,
        0x00, 0x00, 0x00, 0x04,
        'j' as u8, 'o' as u8, 'h' as u8, 'n' as u8,
        0x00, 0x00, 0x00, 0x06,
        '(' as u8, 'q' as u8, 'u' as u8, 'i' as u8,
        't' as u8, ')' as u8, 0x00, 0x00,
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
        data: file_contents.into(),
    };

    let bytes = to_bytes(&initial_file).unwrap();
    let length = bytes.len() as u64;
    let mut cursor = Cursor::new(bytes);

    let recovered_file = from_reader(&mut cursor).unwrap();

    assert_eq!(initial_file, recovered_file);
    assert_eq!(cursor.position(), length);
}
