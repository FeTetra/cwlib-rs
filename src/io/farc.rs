use std::io::{self, SeekFrom};

use crate::io::serdes::{BufSerializer, Deserializer, Serializer, SizedDeserializer};
use crate::io::helpers::filehelper;

use crate::types::farc::{FARCFooter, FARCTableEntry, FileArchive};

// TODO: Add deserialization for array types and use less strings and vecs so we can calculate static struct sizes on the fly

impl Deserializer for FileArchive {
    fn deserialize_from<R: std::io::Read + std::io::Seek, B: byteorder::ByteOrder>(reader: &mut R) -> std::io::Result<Self> {
        // Calculate offset to FARC footer
        let file_size = filehelper::calc_file_size_from_reader(reader)?;

        let footer_offset = file_size - 8; // Footer size is 8 bytes
        reader.seek(SeekFrom::Start(footer_offset))?;

        // Deserialize footer
        let entry_count = u32::deserialize_from::<_, B>(reader)?;
        let magic = String::deserialize_from::<_>(reader, 4)?;

        let footer = FARCFooter {
            entry_count,
            magic,
        };

        // Calculate offset to FARC table
        let table_offset = footer_offset - (28 * entry_count as u64); // Table entry size is 28 bytes
        reader.seek(SeekFrom::Start(table_offset))?;

        // Deserialize table
        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            let file_hash = Vec::deserialize_from::<_>(reader, 0x14)?;
            let file_offset = u32::deserialize_from::<_, B>(reader)?;
            let file_size = u32::deserialize_from::<_, B>(reader)?;

            let entry = FARCTableEntry {
                file_hash,
                file_offset,
                file_size,
            };

            entries.push(entry);
        }

        Ok(FileArchive {
            footer,
            entries,
        })
    }
}

// This requires the library user to write files to the top prior to using this to serialize table and footer
impl Serializer for FileArchive {
    fn serialize_to<W: std::io::Write + std::io::Seek, B: byteorder::ByteOrder>(&self, writer: &mut W) -> std::io::Result<()> {
        // Serialize table
        for entry in &self.entries {
            entry.file_hash.serialize_to::<_>(writer)?;
            entry.file_offset.serialize_to::<_, B>(writer)?;
            entry.file_size.serialize_to::<_, B>(writer)?;
        }

        self.footer.entry_count.serialize_to::<_, B>(writer)?;
        self.footer.magic.serialize_to::<_>(writer)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum ArchiveError {
    NotFound,
}

// This should use a hashmap, but some typing needs to be reworked later anyways
impl FileArchive {
    pub fn get_entry_by_hash(&self, hash: &[u8]) -> Result<&FARCTableEntry, ArchiveError> {
        for entry in &self.entries {
            if hash == entry.file_hash {
                return Ok(entry);
            }
        }

        Err(ArchiveError::NotFound)
    }
}

impl FARCTableEntry {
    pub fn get_file<R: std::io::Read + std::io::Seek, W: std::io::Write>(&self, reader: &mut R, writer: &mut W) -> io::Result<()> {
        let mut buffer = vec![0u8; self.file_size as usize];
        reader.seek(SeekFrom::Start(self.file_offset as u64))?;
        reader.read_exact(&mut buffer)?;

        writer.write_all(&buffer)?;

        Ok(())
    }
}
