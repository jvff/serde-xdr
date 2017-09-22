use std::io::Cursor;

use super::{from_reader, to_bytes};

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
    data: Vec<u8>,
}

#[test]
fn serialize_deserialize() {
    let initial_file = File {
        filename: "sillyprog".to_string(),
        filetype: FileType::Exec("lisp".to_string()),
        owner: "john".to_string(),
        data: "(quit)".into(),
    };

    let bytes = to_bytes(&initial_file).unwrap();
    let length = bytes.len() as u64;
    let mut cursor = Cursor::new(bytes);

    let recovered_file = from_reader(&mut cursor).unwrap();

    assert_eq!(initial_file, recovered_file);
    assert_eq!(cursor.position(), length);
}
