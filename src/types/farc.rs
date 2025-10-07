#[derive(Debug, PartialEq)]
pub struct FARCFooter {
    pub entry_count: u32,
    pub magic: String,
}

#[derive(Debug, PartialEq)]
pub struct FARCTableEntry {
    pub file_hash: Vec<u8>,
    pub file_offset: u32,
    pub file_size: u32,
}

#[derive(Debug, PartialEq)]
pub struct FileArchive {
    pub entries: Vec<FARCTableEntry>,
    pub footer: FARCFooter,
}

impl FARCFooter {
    pub fn print_farc_footer(&self) {
        println!("Entry Count: {}", self.entry_count);
        println!("Magic Number: {}", self.magic);
    }
}

impl FARCTableEntry {
    pub fn print_farc_table_entry(&self) {
        println!("File Hash: {:?}", self.file_hash);
        println!("File Offset: {:?}", self.file_offset);
        println!("File Size: {}", self.file_size);
    }
}

impl FileArchive {
    // Probably only useful for serialization debugging, no good reason to call, but sure
    pub fn print_file_archive(&self) {
        self.footer.print_farc_footer();
        for entry in &self.entries {
            entry.print_farc_table_entry();
        }
    }
}
