use {
    super::super::{
        tests::{Value, Visitor},
        Deserializer,
    },
    serde::Deserializer as _,
    std::io::Cursor,
};

#[test]
fn deserialize_struct() {
    static FIELD_NAMES: [&str; 3] = ["bool", "string", "integer"];

    let mut cursor = Cursor::new(vec![
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 'H' as u8, 'i' as u8,
        '!' as u8, 0x00, 0xff, 0xff, 0xff, 0xfe,
    ]);

    let result = Deserializer::new(&mut cursor)
        .deserialize_struct("struct", &FIELD_NAMES, Visitor)
        .unwrap();

    let expected_result = Value::Sequence(vec![
        Value::Bool(true),
        Value::String("Hi!".to_string()),
        Value::Integer32(-2),
    ]);

    assert_eq!(cursor.position(), 16);
    assert_eq!(result, expected_result);
}
