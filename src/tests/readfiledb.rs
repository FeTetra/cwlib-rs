use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter, Read, Write};

use crate::{serialization, types};
use crate::enums;

use crate::serialization::filedb::BinaryDeserialize;
use crate::serialization::filedb::BinarySerialize;

fn file_db_serialize() -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./blurayguids.map")?;
    let mut writer = BufWriter::new(file);

    let filedb = types::filedb::FileDBHeader::new(
        enums::filedbrevision::FileDBRevision::LBP1Or2, 
        0
    );

    filedb.serialize(&mut writer)?;

    drop(writer);

    Ok(())
}

fn file_db_deserialize() -> io::Result<types::filedb::FileDBHeader> {
    let file = File::open("./blurayguids.map")?;
    let mut reader = BufReader::new(file);

    let mut filedb = types::filedb::FileDBHeader::default();
    filedb.deserialize(&mut reader)?;

    filedb.print_filedb();

    drop(reader);

    Ok(filedb)
}

// I don't feel like writing proper asserts so pretend this isn't that bad
#[cfg(test)]
mod tests {
    use crate::{ 
        tests::readfiledb::{file_db_deserialize, file_db_serialize}, 
        types::filedb::FileDBHeader,
        enums::filedbrevision::FileDBRevision,
    };

    #[test]
    fn test_file_db_serialization() {
        let test_filedb = FileDBHeader::new(
            FileDBRevision::LBP1Or2, 
            0,
        );

        let write = file_db_serialize();
        assert!(write.is_ok());

        let read = file_db_deserialize();
        assert!(read.is_ok());
        assert_eq!(read.unwrap(), test_filedb);
    }
}