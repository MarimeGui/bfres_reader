use error::{RelativePointerDataInvalid, UserDataNotEmpty};
use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};
use std::marker::PhantomData;

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

#[derive(Clone, Copy)]
pub struct Pointer {
    pub location: Option<u64>,
    pub points_to: i32,
}

pub struct BufferInfo {
    pub size: u32,
    pub stride: u16,
    pub buffering_count: u16,
    pub data_offset: Pointer,
}

impl Pointer {
    pub fn new_abs(offset: i32) -> Pointer {
        Pointer {
            location: None,
            points_to: offset,
        }
    }
    pub fn new_rel(location: u64, points_to: i32) -> Pointer {
        Pointer {
            location: Some(location),
            points_to,
        }
    }
    pub fn read_new_rel_i32_be<R: Read + Seek>(reader: &mut R) -> Result<Pointer, Box<Error>> {
        Ok(Pointer {
            location: Some(reader.seek(SeekFrom::Current(0))?),
            points_to: reader.read_be_to_i32()?,
        })
    }
    pub fn get_abs_pos(&self) -> Result<u64, Box<Error>> {
        let temp: i64 = match self.location {
            Some(a) => a as i64,
            None => 0i64,
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

impl Importable for BufferInfo {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<BufferInfo, Box<Error>> {
        let data_pointer = reader.read_be_to_u32()?;
        if data_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: data_pointer,
                data_desc: "Data Pointer".to_string(),
            }));
        }
        let size = reader.read_be_to_u32()?;
        let handle = reader.read_be_to_u32()?;
        if handle != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: handle,
                data_desc: "Handle".to_string(),
            }));
        }
        let stride = reader.read_be_to_u16()?;
        let buffering_count = reader.read_be_to_u16()?;
        let context_pointer = reader.read_be_to_u32()?;
        if context_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: context_pointer,
                data_desc: "Context Pointer".to_string(),
            }));
        }
        let data_offset = Pointer::read_new_rel_i32_be(reader)?;
        Ok(BufferInfo {
            size,
            stride,
            buffering_count,
            data_offset,
        })
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
            break;
        } else {
            bytes.push(current_byte[0]);
        }
    }
    Ok(String::from_utf8(bytes)?)
}
