extern crate ez_io;

mod error;
mod util;
pub mod fres;
pub mod fmdl;
pub mod ftex;

use ez_io::ReadE;
use std::io::{Read, Seek, SeekFrom};
use std::error::Error;
use std::marker::Sized;
use util::Pointer;
use util::read_text_entry;

pub trait Importable where Self: Sized {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<Error>>;
}

pub struct IndexGroup {
    pub entries: Vec<IndexGroupEntry>
}

pub struct IndexGroupEntry {
    pub search_value: u32,
    pub left_index: u16,
    pub right_index: u16,
    pub name_pointer: Pointer,
    pub data_pointer: Pointer
}

impl Importable for IndexGroup {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<IndexGroup, Box<Error>> {
        let end_of_group_absolute_pos = u64::from(reader.read_be_to_u32()?) + reader.seek(SeekFrom::Current(0))?;
        let nb_entries = reader.read_be_to_i32()?;
        let mut entries: Vec<IndexGroupEntry> = Vec::with_capacity(nb_entries as usize);
        reader.seek(SeekFrom::Current(16))?;  // Skip root entry
        for _ in 0..nb_entries {
            entries.push(IndexGroupEntry::import(reader)?);
        }
        if reader.seek(SeekFrom::Current(0))? > end_of_group_absolute_pos {
            panic!();
        }
        Ok(IndexGroup {
            entries
        })
    }
}

impl Importable for IndexGroupEntry {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<IndexGroupEntry, Box<Error>> {
        let search_value = reader.read_be_to_u32()?;
        let left_index = reader.read_be_to_u16()?;
        let right_index = reader.read_be_to_u16()?;
        let name_pointer = Pointer::read_new_rel_i32_be(reader)?;
        let data_pointer = Pointer::read_new_rel_i32_be(reader)?;
        Ok(IndexGroupEntry {
            search_value,
            left_index,
            right_index,
            name_pointer,
            data_pointer
        })
    }
}

impl IndexGroupEntry {
    pub fn get_name<R: Read + Seek>(&self, reader: &mut R) -> Result<String, Box<Error>> {
        self.name_pointer.seek_abs_pos(reader)?;
        Ok(read_text_entry(reader)?)
    }
}

/* General Ideas:
File Type: BFRES
Data Type: FRES
Compression Method: Yaz0
Data inside: Sub-files

String Table Length + Offset (Header)
 @ Offset + 0x1C
 String Length
 String Data

File Offsets + Count, one for each sub-file type (12 types) (Header) -> Sub-file Index Group Offset Table
 @ Offset + Position where you got it from -> Sub-file Index Group Header
  Length of group
  Number of entries / sub-files (should match count in header)
   @ each entry -> Sub-file index group Entry
    Search Value
    Left Index
    Right Index
    Name Pointer
    Data pointer

Sub-file index group entry -> Sub-file absolute offset
*/
