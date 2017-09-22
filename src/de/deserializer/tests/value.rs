use ordered_float::OrderedFloat;

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    Nothing,
    Bool(bool),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    UnsignedInteger8(u8),
    UnsignedInteger16(u16),
    UnsignedInteger32(u32),
    UnsignedInteger64(u64),
    Float(OrderedFloat<f32>),
    Double(OrderedFloat<f64>),
    String(String),
    Bytes(Vec<u8>),
    None,
    Unit,
    Sequence(Vec<Value>),
    Enum(usize, Box<Value>)
}
