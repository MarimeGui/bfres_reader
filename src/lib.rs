extern crate ez_io;

mod error;
mod util;
pub mod fres;
pub mod fmdl;
pub mod ftex;
pub mod fska;
pub mod fshu;
pub mod ftxp;
pub mod fvis;
pub mod fsha;
pub mod fscn;
pub mod embedded;

use ez_io::ReadE;
use std::io::{Read, Seek, SeekFrom};
use std::error::Error;
use std::marker::{Sized, PhantomData};
use util::Pointer;
use util::read_text_entry;

pub trait Importable where Self: Sized {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<Error>>;
}

pub struct IndexGroup<I: Importable> {
    pub entries: Vec<IndexGroupEntry<I>>
}

pub struct IndexGroupEntry<I: Importable> {
    pub search_value: u32,
    pub left_index: u16,
    pub right_index: u16,
    pub name_pointer: Pointer,
    pub data_pointer: Pointer,
    data_type: PhantomData<I>
}

pub struct DataArray<I: Importable> {
    pub entries: Vec<DataArrayEntry<I>>
}

pub struct DataArrayEntry<I: Importable> {
    pub data_pointer: Pointer,
    data_type: PhantomData<I>
}

impl <I: Importable> Importable for IndexGroup<I> {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<IndexGroup<I>, Box<Error>> {
        let end_of_group_absolute_pos = u64::from(reader.read_be_to_u32()?) + reader.seek(SeekFrom::Current(0))?;
        let nb_entries = reader.read_be_to_i32()?;
        let mut entries: Vec<IndexGroupEntry<I>> = Vec::with_capacity(nb_entries as usize);
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

impl <I: Importable> Importable for IndexGroupEntry<I> {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<IndexGroupEntry<I>, Box<Error>> {
        let search_value = reader.read_be_to_u32()?;
        let left_index = reader.read_be_to_u16()?;
        let right_index = reader.read_be_to_u16()?;
        let name_pointer = Pointer::read_new_rel_i32_be(reader)?;
        let data_pointer = Pointer::read_new_rel_i32_be(reader)?;
        let data_type: PhantomData<I> = PhantomData {};
        Ok(IndexGroupEntry {
            search_value,
            left_index,
            right_index,
            name_pointer,
            data_pointer,
            data_type
        })
    }
}

impl <I: Importable> IndexGroupEntry<I> {
    pub fn get_name<R: Read + Seek>(&self, reader: &mut R) -> Result<String, Box<Error>> {
        self.name_pointer.seek_abs_pos(reader)?;
        Ok(read_text_entry(reader)?)
    }
    pub fn get_data<R: Read + Seek>(&self, reader: &mut R) -> Result<I, Box<Error>> {
        self.data_pointer.seek_abs_pos(reader)?;
        Ok(I::import(reader)?)
    }
}

impl <I: Importable> DataArray<I> {
    pub fn new<S: Seek>(seeker: &mut S, every: u32, amount: u32) -> Result<DataArray<I>, Box<Error>> {
        let mut entries: Vec<DataArrayEntry<I>> = Vec::with_capacity(amount as usize);
        for _ in 0..amount {
            entries.push(DataArrayEntry::new(seeker)?);
            seeker.seek(SeekFrom::Current(every as i64))?;
        }
        Ok(DataArray {
            entries
        })
    }
}

impl <I: Importable> DataArrayEntry<I> {
    pub fn new<S: Seek>(seeker: &mut S) -> Result<DataArrayEntry<I>, Box<Error>> {
        let ptr = Pointer::new_abs(seeker.seek(SeekFrom::Current(0))? as i32);
        let data_type: PhantomData<I> = PhantomData {};
        Ok(DataArrayEntry {
            data_pointer: ptr,
            data_type
        })
    }
    pub fn get_data<R: Read + Seek>(&self, reader: &mut R) -> Result<I, Box<Error>> {
        self.data_pointer.seek_abs_pos(reader)?;
        Ok(I::import(reader)?)
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
