use byteorder::WriteBytesExt;
use serde::ser;
use serde::ser::Serialize;

use super::errors::{Error, Result};

pub struct Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    writer: &'w mut W
}

impl<'w, W> Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    pub fn new(writer: &'w mut W) -> Self {
        Serializer { writer }
    }
}

mod serializer;

impl<'w, W> ser::SerializeSeq for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("seq is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("seq is not supported")
    }
}

impl<'w, W> ser::SerializeTuple for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("tuple is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("tuple is not supported")
    }
}

impl<'w, W> ser::SerializeTupleStruct for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("tuple_struct is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("tuple_struct is not supported")
    }
}

impl<'w, W> ser::SerializeTupleVariant for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("tuple_struct is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("tuple_variant is not supported")
    }
}

impl<'w, W> ser::SerializeMap for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("map is not supported")
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("map is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("map is not supported")
    }
}

impl<'w, W> ser::SerializeStruct for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("struct is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("struct is not supported")
    }
}

impl<'w, W> ser::SerializeStructVariant for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("struct_variant is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("struct_variant is not supported")
    }
}

pub fn to_bytes() -> Vec<u8> {
    Vec::new()
}
