use std::fmt::Display;
use std::io;
use std::io::Write;

use byteorder::WriteBytesExt;
use serde::ser;
use serde::ser::Serialize;

pub use self::errors::{CompatSerializationError, Result, SerializationError};

/// Serializer for the XDR format.
///
/// Structure that holds a mutable borrow of the writer it serializes data to.
/// It has an implementation of `serde::Serializer` so that it can serialize
/// data into its XDR representation.
pub struct Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    writer: &'w mut W,
}

impl<'w, W> Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    /// Create a new instance that serializes data into the given generic
    /// writer.
    pub fn new(writer: &'w mut W) -> Self {
        Serializer { writer }
    }

    fn serialize_failure<T, S>(
        type_name: &str,
        value: T,
        error: S,
    ) -> CompatSerializationError
    where
        T: Display,
        S: Into<CompatSerializationError>,
    {
        SerializationError::Failure {
            what: format!("a value {} of type {}", value, type_name),
            cause: Box::new(error.into()),
        }
        .into()
    }

    fn serialize_opaque_failure<S>(
        length: usize,
        error: S,
    ) -> SerializationError
    where
        S: Into<CompatSerializationError>,
    {
        SerializationError::Failure {
            what: format!("opaque data of length {}", length),
            cause: Box::new(error.into()),
        }
    }

    fn io_error<T>(
        type_name: &str,
        value: T,
        error: io::Error,
    ) -> SerializationError
    where
        T: Display,
    {
        SerializationError::IoError {
            what: format!("a value {} of type {}", value, type_name),
            cause: error,
        }
    }

    fn serialize_opaque_io_error(
        length: usize,
        error: io::Error,
    ) -> SerializationError {
        SerializationError::IoError {
            what: format!("opaque data of length {}", length),
            cause: error,
        }
    }
}

mod serializer;

impl<'w, W> ser::SerializeMap for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = CompatSerializationError;

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

/// Serialize data into a vector of bytes.
///
/// Serializes a generic data type into a new instance of `Vec<u8>`.
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut bytes = Vec::new();

    value.serialize(Serializer::new(&mut bytes))?;

    Ok(bytes)
}

/// Serialize data through a generic writer.
///
/// Serializes a generic data type through a borrowed instance that implements
/// `Write`.
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    value.serialize(Serializer::new(writer))?;

    Ok(())
}

mod errors;
