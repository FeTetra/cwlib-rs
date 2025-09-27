use std::io::{self, Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use std::convert::TryFrom;

use crate::enums::filedbrevision::FileDBRevision;
use crate::types::filedb::FileDBHeader;

// TODO: Move traits (am lazy)

pub trait BinaryDeserialize: Sized {
    fn deserialize<R: Read>(&mut self, reader: &mut R) -> io::Result<()>;
}

// This might be weird, maybe instead return a new FileDBHeader?
impl BinaryDeserialize for FileDBHeader {
    fn deserialize<R: Read>(&mut self, reader: &mut R) -> io::Result<()> {
        let raw = reader.read_u32::<BigEndian>()?;
        self.db_revision = FileDBRevision::try_from(raw).unwrap();
        self.entry_count = reader.read_u32::<BigEndian>()?;

        Ok(())
    }
}

pub trait BinarySerialize: Sized {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}

impl BinarySerialize for FileDBHeader {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<BigEndian>(self.db_revision as u32)?;
        writer.write_u32::<BigEndian>(self.entry_count)?;

        Ok(())
    }
}
