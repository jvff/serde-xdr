use std::fmt;
use std::fmt::Formatter;
use std::io::Cursor;

use serde::de;
use serde::Deserializer as SerdeDeserializer;

use super::Deserializer;

#[derive(Debug, Eq, PartialEq)]
enum Value {
    Integer8(i8),
}

struct Visitor;

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "unknown")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Integer8(value))
    }
}

#[test]
fn deserialize_i8() {
    let mut cursor = Cursor::new(vec![0xff, 0xff, 0xff, 0xfe]);

    let result =
        Deserializer::new(&mut cursor).deserialize_i8(Visitor).unwrap();

    assert_eq!(result, Value::Integer8(-2));
}
