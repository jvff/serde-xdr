use ordered_float::OrderedFloat;

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    Bool(bool),
    Bytes(Vec<u8>),
    Char(char),
    Double(OrderedFloat<f64>),
    Enum(usize, Box<Value>),
    Float(OrderedFloat<f32>),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    None,
    Nothing,
    Sequence(Vec<Value>),
    String(String),
    Unit,
    UnsignedInteger8(u8),
    UnsignedInteger16(u16),
    UnsignedInteger32(u32),
    UnsignedInteger64(u64),
}
