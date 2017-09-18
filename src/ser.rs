use std::io::Write;

pub struct Serializer<'w, W>
where
    W: Write + 'w,
{
    writer: &'w W
}

impl<'w, W> Serializer<'w, W>
where
    W: Write + 'w,
{
    pub fn new(writer: &'w W) -> Self {
        Serializer { writer }
    }
}

pub fn to_bytes() -> Vec<u8> {
    Vec::new()
}
