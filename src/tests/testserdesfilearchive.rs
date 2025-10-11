#[cfg(test)]
mod tests {
    use crate::tests::files::{file_archive_deserialize, file_archive_serialize};
    use crate::types::farc::{FileArchive, FARCTableEntry, FARCFooter};

    #[test]
    fn test_farc_serdes() {
        let test_entry1 = FARCTableEntry {
            file_hash: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
            file_offset: 0x0,
            file_size: 0x1,
        };
        let test_entry2 = FARCTableEntry {
            file_hash: vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
            file_offset: 0x1,
            file_size: 0x2,
        };
        let test_entry3 = FARCTableEntry {
            file_hash: vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
            file_offset: 0x2,
            file_size: 0x3,
        };

        let test_footer = FARCFooter {
            entry_count: 3,
            magic: "FARC".to_string(),
        };

        let test_farc = FileArchive {
            entries: vec![test_entry1, test_entry2, test_entry3],
            footer: test_footer,
        };

        let write = file_archive_serialize(&test_farc, "./data.farc");
        assert!(write.is_ok());

        let read = file_archive_deserialize("./data.farc");
        assert!(read.is_ok());

        let farc = read.unwrap();
        assert_eq!(farc, test_farc);
        farc.print_file_archive();
    }
}
