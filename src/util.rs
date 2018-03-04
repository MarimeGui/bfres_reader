use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};
use error::RelativePointerDataInvalid;

#[derive(Clone, Copy)]
pub struct Pointer {
    pub location: Option<u64>,
    pub points_to: i32
}

impl Pointer {
    pub fn new_abs(offset: i32) -> Pointer {
        Pointer {
            location: None,
            points_to: offset
        }
    }
    pub fn new_rel(location: u64, points_to: i32) -> Pointer {
        Pointer {
            location: Some(location),
            points_to
        }
    }
    pub fn read_new_rel_i32_be<R: Read + Seek>(reader: &mut R) -> Result<Pointer, Box<Error>> {
        Ok(Pointer {
            location: Some(reader.seek(SeekFrom::Current(0))?),
            points_to: reader.read_be_to_i32()?
        })
    }
    pub fn get_abs_pos(&self) -> Result<u64, Box<Error>> {
        let temp: i64 = match self.location {
            Some(a) => a as i64,
            None => 0i64
        } + i64::from(self.points_to);
        if temp < 0 {
            return Err(Box::new(RelativePointerDataInvalid {}));
        };
        Ok(temp as u64)
    }
    pub fn seek_abs_pos<S: Seek>(&self, seeker: &mut S) -> Result<(), Box<Error>> {
        seeker.seek(SeekFrom::Start(self.get_abs_pos()?))?;
        Ok(())
    }
}

pub fn align_on_4_bytes<R: Read + Seek>(reader: &mut R) -> Result<(), Box<Error>> {
    let pos = reader.seek(SeekFrom::Current(0))?;
    if pos % 4 != 0 {
        reader.seek(SeekFrom::Current((4 - (pos % 4)) as i64))?;
    }
    Ok(())
}

pub fn read_text_entry<R: Read + Seek>(reader: &mut R) -> Result<String, Box<Error>> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut current_byte: [u8; 1] = [0u8; 1];
    loop {
        reader.read_exact(&mut current_byte)?;
        if current_byte[0] == 0u8 {
            break
        } else {
            bytes.push(current_byte[0]);
        }
    }
    Ok(String::from_utf8(bytes)?)
}