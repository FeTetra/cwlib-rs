use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter};
use byteorder::BigEndian;

use crate::io::serdes::{Deserializer, Serializer};
use crate::types::filedb::{FileDB};

/*
This file technically contains okayish reference implementations of
FileDB::serialize_to() and FileDB::deserialize_from(), in the next two 
functions. Docs someday trust:tm:.
*/

fn file_db_serialize(filedb: &FileDB, path: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let mut writer = BufWriter::new(file);

    filedb.serialize_to::<_, BigEndian>(&mut writer)?;

    drop(writer);

    Ok(())
}

fn file_db_deserialize(path: &str) -> io::Result<FileDB> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let filedb = FileDB::deserialize_from::<_, BigEndian>(&mut reader)?;

    filedb.print_filedb(); // You probably wouldn't really want to use this, we only have 3 entries, though

    drop(reader);

    Ok(filedb)
}

// I don't feel like writing proper asserts so pretend this isn't that bad
#[cfg(test)]
mod tests {
    use crate::{ 
        tests::readfiledb::{file_db_deserialize, file_db_serialize},
        types::filedb::{FileDBHeader, FileDBEntry, FileDB},
        enums::filedbrevision::FileDBRevision,
    };

    #[test]
    fn test_file_db_serdes() {
        // Begin bullshit
        let header = FileDBHeader {
            db_revision: FileDBRevision::LBP1Or2,
            entry_count: 3,
        };

        // Aw hell naw :disaster:
        let test_entry1 = FileDBEntry {
            path_size: 4,
            path: "name".to_string(),
            date: 1759552269,
            size: 1,
            hash: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
            guid: 1
        };
        let test_entry2 = FileDBEntry {
            path_size: 13,
            path: "/path/to/file".to_string(),
            date: 1759552461,
            size: 10,
            hash: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
            guid: 2
        };
            let test_entry3 = FileDBEntry {
            path_size: 12,
            path: "somefile.txt".to_string(),
            date: 1759552536,
            size: 100,
            hash: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
            guid: 3
        };
        let entries = vec![test_entry1, test_entry2, test_entry3];

        let filedb = FileDB {
            header: header,
            entries: entries,
        };
        // End bullshit

        let write = file_db_serialize(&filedb, "./blurayguids.map");
        assert!(write.is_ok());

        let read = file_db_deserialize("./blurayguids.map");
        assert!(read.is_ok());
        assert_eq!(read.unwrap(), filedb);
    }
}