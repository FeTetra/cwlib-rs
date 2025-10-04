use crate::enums::filedbrevision::FileDBRevision;

#[derive(Debug, PartialEq)]
pub struct FileDBHeader {
    pub db_revision: FileDBRevision,
    pub entry_count: u32,
}

#[derive(Debug, PartialEq)]
pub struct FileDBEntry {
    pub path_size: u16,
    pub path: String,
    pub date: u32,
    pub size: u32,
    pub hash: Vec<u8>, // I don't like this for many reasons but I can deal with it later
    pub guid: u32,
}

#[derive(Debug, PartialEq)]
pub struct FileDB {
    pub header: FileDBHeader,
    pub entries: Vec<FileDBEntry>
}

impl FileDBHeader {
    pub fn print_dbheader(&self) {
        println!("DB Revision: {:?}", self.db_revision);
        println!("DB Entry Count: {}", self.entry_count);
    }
}

impl Default for FileDBHeader {
    fn default() -> Self {
        FileDBHeader { 
            db_revision: (FileDBRevision::Unknown), 
            entry_count: (0),
        }
    }
}

impl FileDBEntry {
    pub fn print_dbentry(&self) {
        println!("Path: {}", self.path);
        println!("Date: {}", self.date);
        println!("Size: {}", self.size);
        println!("Hash: {:?}", self.hash);
        println!("GUID: {}", self.guid);
    }
}

impl FileDB {
    // Really no reason to call this but go ahead I guess
    pub fn print_filedb(&self) {
        self.header.print_dbheader();
        for entry in &self.entries {
            entry.print_dbentry();
        }
    }
}
