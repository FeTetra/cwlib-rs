use std::io::{self, Read, Seek, Write};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};

// Traits to implement deserialization for arbitrary types

pub trait Deserializer: Sized {
    fn deserialize_from<R: Read + Seek, B: ByteOrder>(reader: &mut R) -> io::Result<Self>;
}

pub trait SizedDeserializer: Sized {
    fn deserialize_from<R: Read>(reader: &mut R, size: usize) -> io::Result<Self>;
}

// Deserialization trait implementations for primitives

impl Deserializer for u16 {
    fn deserialize_from<R: Read, B: ByteOrder>(reader: &mut R) -> io::Result<Self> {
        reader.read_u16::<B>()
    }
}

impl Deserializer for u32 {
    fn deserialize_from<R: Read, B: ByteOrder>(reader: &mut R) -> io::Result<Self> {
        reader.read_u32::<B>()
    }
}

impl SizedDeserializer for String {
    fn deserialize_from<R: Read>(reader: &mut R, size: usize) -> io::Result<Self> {
        let mut buf = vec![0u8; size];
        reader.read_exact(&mut buf)?;
        String::from_utf8(buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl SizedDeserializer for Vec<u8> {
    fn deserialize_from<R: Read>(reader: &mut R, size: usize) -> io::Result<Self> {
        let mut buf = vec![0u8; size];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

// Traits to implement serialization for arbitrary types

pub trait Serializer: Sized {
    fn serialize_to<W: Write + Seek, B: ByteOrder>(&self, writer: &mut W) -> io::Result<()>;
}

pub trait BufSerializer: Sized {
    fn serialize_to<W: Write + Seek>(&self, writer: &mut W) -> io::Result<()>;
}

// Serialization trait implementations for primitives

impl Serializer for u16 {
    fn serialize_to<W: Write, B: ByteOrder>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u16::<B>(*self)
    }
}

impl Serializer for u32 {
    fn serialize_to<W: Write, B: ByteOrder>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<B>(*self)
    }
}

impl BufSerializer for String {
    fn serialize_to<W: Write + Seek>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self.as_bytes())
    }
}

impl BufSerializer for Vec<u8> {
    fn serialize_to<W: Write + Seek>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self)
    }
}
