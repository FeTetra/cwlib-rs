use std::io::{self, Read, Write, Seek};
use byteorder::ByteOrder;
use std::convert::TryFrom;

use crate::io::serdes::{Deserializer, Serializer, SizedDeserializer, BufSerializer};

use crate::enums::filedbrevision::FileDBRevision;
use crate::types::filedb::{FileDB, FileDBEntry, FileDBHeader};

impl Deserializer for FileDB {
    fn deserialize_from<R: Read + Seek, B: ByteOrder>(reader: &mut R) -> io::Result<Self> {
        // Deserialize header
        let raw = u32::deserialize_from::<_, B>(reader)?;
        let db_revision = FileDBRevision::try_from(raw).unwrap();
        let entry_count = u32::deserialize_from::<_, B>(reader)?;

        // Deserialize entries
        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            if db_revision != FileDBRevision::LBP3 {
                reader.seek_relative(2)?;
            }

            let path_size = u16::deserialize_from::<_, B>(reader)?;
            let path = String::deserialize_from::<_>(reader, path_size as usize)?;

            if db_revision == FileDBRevision::LBP1Or2 {
                reader.seek_relative(4)?;
            }

            let date = u32::deserialize_from::<_, B>(reader)?;
            let size = u32::deserialize_from::<_, B>(reader)?;
            let hash = Vec::deserialize_from::<_>(reader, 0x14)?;
            let guid = u32::deserialize_from::<_, B>(reader)?;

            let entry = FileDBEntry {
                path_size,
                path,
                date,
                size,
                hash,
                guid,
            };

            entries.push(entry);
        }

        Ok(FileDB {
            header: FileDBHeader {
                db_revision,
                entry_count,
            },
            entries,
        })
    }
}

impl Serializer for FileDB {
    fn serialize_to<W: Write + Seek, B: ByteOrder>(&self, writer: &mut W) -> io::Result<()> {
        // Serialize header
        (self.header.db_revision as u32).serialize_to::<_, B>(writer)?;
        self.header.entry_count.serialize_to::<_, B>(writer)?;

        // Serialize entries
        for entry in &self.entries {
            if self.header.db_revision != FileDBRevision::LBP3 {
                writer.seek_relative(2)?;
            }

            entry.path_size.serialize_to::<_, B>(writer)?;
            entry.path.serialize_to(writer)?;

            if self.header.db_revision == FileDBRevision::LBP1Or2 {
                writer.seek_relative(4)?;
            }

            entry.date.serialize_to::<_, B>(writer)?;
            entry.size.serialize_to::<_, B>(writer)?;
            entry.hash.serialize_to(writer)?;
            entry.guid.serialize_to::<_, B>(writer)?;
        }

        Ok(())
    }
}