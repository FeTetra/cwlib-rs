use crate::enums::filedbrevision::FileDBRevision;

#[derive(Debug, PartialEq)]
pub struct FileDBHeader {
    pub db_revision: FileDBRevision,
    pub entry_count: u32,
}

impl FileDBHeader {
    pub fn new(db_revision: FileDBRevision, entry_count: u32) -> Self {
        Self { db_revision, entry_count }
    }

    pub fn print_filedb(&self) {
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
