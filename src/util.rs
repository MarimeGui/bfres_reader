use std::error::Error;
use std::io::{Read, Seek, SeekFrom};
use error::RelativePointerDataInvalid;

#[derive(Clone, Copy)]
pub struct RelativePointer {
    pub location: u64,
    pub points_to: i64
}

impl RelativePointer {
    pub fn absolute_position(&self) -> Result<u64, Box<Error>> {
        let temp: i64 = self.location as i64 + self.points_to;
        if temp < 0 {
            return Err(Box::new(RelativePointerDataInvalid {}));
        };
        Ok(temp as u64)
    }
}

pub fn align_on_4_bytes<R: Read + Seek>(reader: &mut R) -> Result<(), Box<Error>> {
    let pos = reader.seek(SeekFrom::Current(0))?;
    if pos % 4 != 0 {
        reader.seek(SeekFrom::Current((4 - (pos % 4)) as i64))?;
    }
    Ok(())
}