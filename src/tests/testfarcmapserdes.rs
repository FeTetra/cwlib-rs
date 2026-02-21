#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufWriter, Write, Cursor};
    use byteorder::BigEndian;

    use crate::enums::filedbrevision::FileDBRevision;
    use crate::io::serdes::{Serializer, SizedDeserializer, Deserializer};
    use crate::types::farc::{FARCFooter, FARCTableEntry, FileArchive};
    use crate::types::filedb::{FileDB, FileDBEntry, FileDBHeader};

    // This is completely miserable and theres almost certainly a better way to do these tests
    // This also could probably replace the testserdes(filedb/filearchive) tests entirely
    #[test] 
    fn test_farc_file_get() -> Result<(), Box<dyn std::error::Error>> {
        // Farc files
        let test_dat1 = "Hello, world!";
        let test_dat2 = "Foo";
        let test_dat3 = "Bar";

        let test_dat1_size = 13;
        let test_dat2_size = 3;
        let test_dat3_size = 3;

        let test_dat1_hash = vec![
            0x94, 0x3A, 0x70, 0x2D, 0x06, 0xF3, 0x45, 
            0x99, 0xAE, 0xE1, 0xF8, 0xDA, 0x8E, 0xF9, 
            0xF7, 0x29, 0x60, 0x31, 0xD6, 0x99, 
        ];
        let test_dat2_hash = vec![
            0x20, 0x1A, 0x6B, 0x30, 0x53, 0xCC, 0x14, 
            0x22, 0xD2, 0xC3, 0x67, 0x0B, 0x62, 0x61, 
            0x62, 0x21, 0xD2, 0x29, 0x09, 0x29, 
        ];
        let test_dat3_hash= vec![
            0xE4, 0x96, 0xFD, 0x20, 0x13, 0x6D, 0x4B, 
            0xB7, 0x82, 0x8E, 0xBB, 0x0A, 0xB9, 0x25, 
            0xB1, 0xBD, 0x97, 0x72, 0x08, 0xE4, 
        ];

        // FARC table entries
        let test_farc_entry1 = FARCTableEntry {
            file_hash: test_dat1_hash.clone(), // Clone the first time because ownership gets moved otherwise
            file_offset: 0x0,
            file_size: test_dat1_size,
        };
        let test_farc_entry2 = FARCTableEntry {
            file_hash: test_dat2_hash.clone(),
            file_offset: test_dat1_size,
            file_size: test_dat2_size,
        };
        let test_farc_entry3 = FARCTableEntry {
            file_hash: test_dat3_hash.clone(),
            file_offset: test_dat1_size + test_dat2_size,
            file_size: test_dat3_size,
        };

        // GUID map entries
        let test_file_db_entry1 = FileDBEntry {
            path_size: 16,
            path: "./helloworld.dat".to_string(),
            date: 1760198010,
            size: test_dat1_size,
            hash: test_dat1_hash,
            guid: 1,
        };
        let test_file_db_entry2 = FileDBEntry {
            path_size: 9,
            path: "./foo.dat".to_string(),
            date: 1760198014,
            size: test_dat2_size,
            hash: test_dat2_hash,
            guid: 2,
        };
        let test_file_db_entry3 = FileDBEntry {
            path_size: 9,
            path: "./bar.dat".to_string(),
            date: 1760198019,
            size: test_dat3_size,
            hash: test_dat3_hash,
            guid: 3,
        };
        
        // GUID map header / FARC footer
        let test_file_db_header = FileDBHeader {
            db_revision: FileDBRevision::LBP1Or2,
            entry_count: 3,
        };
        let test_farc_footer = FARCFooter {
            entry_count: 3,
            magic: "FARC".to_string(),
        };

        // Assemble test FARC and GUID map
        let test_farc = FileArchive {
            entries: vec![test_farc_entry1, test_farc_entry2, test_farc_entry3],
            footer: test_farc_footer,
        };
        let test_file_db = FileDB {
            header: test_file_db_header,
            entries: vec![test_file_db_entry1, test_file_db_entry2, test_file_db_entry3],
        };

        // Serialize the GUID map
        let mut file_db_buf = Cursor::new(Vec::new());
        let mut writer = BufWriter::new(file_db_buf);

        test_file_db.serialize_to::<_, BigEndian>(&mut writer)?;

        writer.flush()?;
        file_db_buf = writer.into_inner()?;
        file_db_buf.set_position(0);
        let mut file_db_reader = BufReader::new(file_db_buf);

        // Serialize the FARC
        let mut farc_buf = Cursor::new(Vec::new());

        writer = BufWriter::new(farc_buf);

        // Manually write file data to FARC
        writer.write_all(test_dat1.as_bytes())?;
        writer.write_all(test_dat2.as_bytes())?;
        writer.write_all(test_dat3.as_bytes())?;

        test_farc.serialize_to::<_, BigEndian>(&mut writer)?;

        writer.flush()?;
        farc_buf = writer.into_inner()?; // Reclaim buffer
        farc_buf.set_position(0);
        let mut farc_reader = BufReader::new(farc_buf);

        // Deserialize everything and use GUID map to read out the files
        let file_db = FileDB::deserialize_from::<_, BigEndian>(&mut file_db_reader)?;
        let farc = FileArchive::deserialize_from::<_, BigEndian>(&mut farc_reader)?;

        file_db.print_filedb();
        farc.print_file_archive();

        for i in 1..4 {
            let err = file_db.get_entry_by_guid(i);
            assert!(err.is_ok()); // Can't just return this error type
            let file_db_entry = err.unwrap();

            let mut file_buf = Cursor::new(Vec::new());
            let mut writer = BufWriter::new(file_buf);

            let err = farc.get_entry_by_hash(&file_db_entry.hash);
            assert!(err.is_ok());
            let farc_entry = err.unwrap();

            farc_entry.get_file(&mut farc_reader, &mut writer)?;

            writer.flush()?;
            file_buf = writer.into_inner()?;
            file_buf.set_position(0);

            let mut reader = BufReader::new(file_buf);
            let read_string = String::deserialize_from(&mut reader, farc_entry.file_size as usize)?;

            assert!(read_string == test_dat1 || read_string == test_dat2 || read_string == test_dat3);

            println!("File data read from farc: {}", read_string);
        }

        Ok(())
    }
}