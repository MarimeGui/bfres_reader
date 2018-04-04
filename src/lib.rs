extern crate ez_io;

pub mod embedded;
mod error;
pub mod fmdl;
pub mod fres;
pub mod fscn;
pub mod fsha;
pub mod fshu;
pub mod fska;
pub mod ftex;
pub mod ftxp;
pub mod fvis;
pub mod swizzle;
mod util;

use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};
use std::marker::{PhantomData, Sized};
use util::Pointer;
use util::read_text_entry;

pub trait Importable
where
    Self: Sized,
{
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<Error>>;
}

pub struct IndexGroup<I: Importable> {
    pub entries: Vec<IndexGroupEntry<I>>,
}

pub struct IndexGroupEntry<I: Importable> {
    pub search_value: u32,
    pub left_index: u16,
    pub right_index: u16,
    pub name_pointer: Pointer,
    pub data_pointer: Pointer,
    data_type: PhantomData<I>,
}

pub struct DataArray<I: Importable> {
    pub entries: Vec<DataArrayEntry<I>>,
}

pub struct DataArrayEntry<I: Importable> {
    pub data_pointer: Pointer,
    data_type: PhantomData<I>,
}

impl<I: Importable> Importable for IndexGroup<I> {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<IndexGroup<I>, Box<Error>> {
        let end_of_group_absolute_pos =
            u64::from(reader.read_be_to_u32()?) + reader.seek(SeekFrom::Current(0))?;
        let nb_entries = reader.read_be_to_i32()?;
        let mut entries: Vec<IndexGroupEntry<I>> = Vec::with_capacity(nb_entries as usize);
        reader.seek(SeekFrom::Current(16))?; // Skip root entry
        for _ in 0..nb_entries {
            entries.push(IndexGroupEntry::import(reader)?);
        }
        if reader.seek(SeekFrom::Current(0))? > end_of_group_absolute_pos {
            panic!();
        }
        Ok(IndexGroup { entries })
    }
}

impl<I: Importable> Importable for IndexGroupEntry<I> {
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
            data_type,
        })
    }
}

impl<I: Importable> IndexGroupEntry<I> {
    pub fn get_name<R: Read + Seek>(&self, reader: &mut R) -> Result<String, Box<Error>> {
        self.name_pointer.seek_abs_pos(reader)?;
        Ok(read_text_entry(reader)?)
    }
    pub fn get_data<R: Read + Seek>(&self, reader: &mut R) -> Result<I, Box<Error>> {
        self.data_pointer.seek_abs_pos(reader)?;
        Ok(I::import(reader)?)
    }
}

impl<I: Importable> DataArray<I> {
    pub fn new<S: Seek>(
        seeker: &mut S,
        every: u32,
        amount: u32,
    ) -> Result<DataArray<I>, Box<Error>> {
        let mut entries: Vec<DataArrayEntry<I>> = Vec::with_capacity(amount as usize);
        for _ in 0..amount {
            entries.push(DataArrayEntry::new(seeker)?);
            seeker.seek(SeekFrom::Current(i64::from(every)))?;
        }
        Ok(DataArray { entries })
    }
}

impl<I: Importable> DataArrayEntry<I> {
    pub fn new<S: Seek>(seeker: &mut S) -> Result<DataArrayEntry<I>, Box<Error>> {
        let ptr = Pointer::new_abs(seeker.seek(SeekFrom::Current(0))? as i32);
        let data_type: PhantomData<I> = PhantomData {};
        Ok(DataArrayEntry {
            data_pointer: ptr,
            data_type,
        })
    }
    pub fn get_data<R: Read + Seek>(&self, reader: &mut R) -> Result<I, Box<Error>> {
        self.data_pointer.seek_abs_pos(reader)?;
        Ok(I::import(reader)?)
    }
}