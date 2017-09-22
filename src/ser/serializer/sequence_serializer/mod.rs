use byteorder::WriteBytesExt;
use serde::{Serialize, Serializer as SerdeSerializer};
use serde::ser::SerializeSeq;

use super::super::Serializer;
use super::super::super::errors::{Error, ErrorKind, Result, ResultExt};

pub struct SequenceSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    serializer: Option<Serializer<'w, W>>,
    current_index: usize,
}

impl<'w, W> SequenceSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    pub fn start(
        length: Option<usize>,
        mut serializer: Serializer<'w, W>,
    ) -> Result<Self> {
        if let Some(length) = length {
            serializer = Self::serialize_length(length, serializer)?;
        } else {
            bail!(ErrorKind::SerializeSequenceWithUnknownLength);
        }

        Ok(SequenceSerializer {
            serializer: Some(serializer),
            current_index: 0,
        })
    }

    fn serialize_length(
        length: usize,
        serializer: Serializer<'w, W>,
    ) -> Result<Serializer<'w, W>> {
        Self::ensure_length_is_valid(length)?;

        serializer.serialize_u32(length as u32)
            .chain_err(|| ErrorKind::SerializeSequenceLength(length))
    }

    fn ensure_length_is_valid(length: usize) -> Result<()> {
        let max_length = u32::max_value() as usize;

        ensure!(length <= max_length, ErrorKind::SequenceTooLong(length));

        Ok(())
    }
}

impl<'w, W> SerializeSeq for SequenceSerializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Serializer<'w, W>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if let Some(serializer) = self.serializer.take() {
            let serializer = value.serialize(serializer).chain_err(|| {
                    ErrorKind::SerializeSequenceElement(self.current_index)
                })?;

            self.current_index += 1;
            self.serializer = Some(serializer);

            Ok(())
        } else {
            bail!(ErrorKind::SerializeSequenceFatalError);
        }
    }

    fn end(self) -> Result<Serializer<'w, W>> {
        if let Some(serializer) = self.serializer {
            Ok(serializer)
        } else {
            bail!(ErrorKind::SerializeSequenceFatalError)
        }
    }
}

#[cfg(test)]
mod tests;
