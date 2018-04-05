use self::attributes::Attributes;
use error::{check_magic_number, UserDataNotEmpty};
use ez_io::ReadE;
use std::error::Error;
use std::io::SeekFrom;
use std::io::{Read, Seek};
use util::{BufferInfo, DataArray, Importable, IndexGroup, Pointer};

pub mod attributes;

pub struct FVTX {
    pub header: Header,
    pub attributes_index_group: IndexGroup<Attributes>,
    pub buffer_info_array: DataArray<BufferInfo>,
}

pub struct Header {
    pub attribute_count: u8,
    pub buffer_info_count: u8,
    pub section_index: u16,
    pub nb_vertices: u32,
    pub vertex_skin_count: u8,
    pub attribute_array_offset: Pointer,
    pub attribute_index_group_offset: Pointer,
    pub buffer_info_array_offset: Pointer,
    pub user_pointer: u32,
}

impl Importable for FVTX {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FVTX, Box<Error>> {
        let header = Header::import(reader)?;
        header.attribute_index_group_offset.seek_abs_pos(reader)?;
        let attributes = IndexGroup::import(reader)?;
        header.buffer_info_array_offset.seek_abs_pos(reader)?;
        let buffer_info_array = DataArray::new(reader, 0x18, u32::from(header.buffer_info_count))?;
        Ok(FVTX {
            header,
            attributes_index_group: attributes,
            buffer_info_array,
        })
    }
}

impl Importable for Header {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Header, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        check_magic_number(magic_number, [b'F', b'V', b'T', b'X'])?;
        let attribute_count = reader.read_to_u8()?;
        let buffer_info_count = reader.read_to_u8()?;
        let section_index = reader.read_be_to_u16()?;
        let nb_vertices = reader.read_be_to_u32()?;
        let vertex_skin_count = reader.read_to_u8()?;
        reader.seek(SeekFrom::Current(3))?;
        let attribute_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let attribute_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let buffer_info_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_pointer: u32 = reader.read_be_to_u32()?;
        if user_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: user_pointer,
                data_desc: "User Pointer".to_string(),
            }));
        }
        Ok(Header {
            attribute_count,
            buffer_info_count,
            section_index,
            nb_vertices,
            vertex_skin_count,
            attribute_array_offset,
            attribute_index_group_offset,
            buffer_info_array_offset,
            user_pointer,
        })
    }
}
