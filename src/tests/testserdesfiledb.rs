// I don't feel like writing proper asserts so pretend this isn't that bad
#[cfg(test)]
mod tests {
    use crate::{ 
        tests::files::{file_db_deserialize, file_db_serialize},
        types::filedb::{FileDBHeader, FileDBEntry, FileDB},
        enums::filedbrevision::FileDBRevision,
    };

    #[test]
    fn test_file_db_serdes() {
        // Begin bullshit
        let test_header = FileDBHeader {
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
        let test_entries = vec![test_entry1, test_entry2, test_entry3];

        let test_filedb = FileDB {
            header: test_header,
            entries: test_entries,
        };
        // End bullshit

        let write = file_db_serialize(&test_filedb, "./blurayguids.map");
        assert!(write.is_ok());

        let read = file_db_deserialize("./blurayguids.map");
        assert!(read.is_ok());

        let filedb = read.unwrap();
        assert_eq!(filedb, test_filedb);
        filedb.print_filedb(); // You probably wouldn't really want to use this, we only have 3 entries, though
    }
}