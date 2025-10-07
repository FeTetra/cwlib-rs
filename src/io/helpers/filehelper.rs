use std::io::{Read, Seek, SeekFrom, Result};

pub fn calc_file_size_from_reader<R: Read + Seek>(reader: &mut R) -> Result<u64> {
    let current_pos = reader.seek(SeekFrom::Current(0))?;
    let end = reader.seek(SeekFrom::End(0))?;
    reader.seek(SeekFrom::Start(current_pos))?;
    Ok(end)
}
