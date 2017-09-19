use byteorder::ReadBytesExt;

pub struct Deserializer<'r, R>
where
    R: ReadBytesExt + 'r,
{
    reader: &'r mut R,
}

impl<'r, R> Deserializer<'r, R>
where
    R: ReadBytesExt + 'r,
{
    pub fn new(reader: &'r mut R) -> Self {
        Deserializer { reader }
    }
}

pub fn from_bytes(_bytes: &[u8]) {}
