use std::io::{self, BufReader, BufWriter, Write};
use std::fs::{File, OpenOptions};
use byteorder::BigEndian;

use crate::io::serdes::{Deserializer, Serializer};
use crate::types::{filedb::FileDB, farc::FileArchive};

pub fn file_archive_serialize(farc: &FileArchive, path: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let mut writer = BufWriter::new(file);

    farc.serialize_to::<_, BigEndian>(&mut writer)?;

    writer.flush()?;
    drop(writer);

    Ok(())
}

pub fn file_archive_deserialize(path: &str) -> io::Result<FileArchive> {
    let file = File::open(path)
        .expect("Failed to open file.");
    let mut reader = BufReader::new(file);

    let farc = FileArchive::deserialize_from::<_, BigEndian>(&mut reader)
        .expect("Failed to parse file archive.");

    drop(reader);

    Ok(farc)
}

pub fn file_db_serialize(filedb: &FileDB, path: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let mut writer = BufWriter::new(file);

    filedb.serialize_to::<_, BigEndian>(&mut writer)?;

    writer.flush()?;
    drop(writer);

    Ok(())
}

pub fn file_db_deserialize(path: &str) -> io::Result<FileDB> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let filedb = FileDB::deserialize_from::<_, BigEndian>(&mut reader)?;

    drop(reader);

    Ok(filedb)
}