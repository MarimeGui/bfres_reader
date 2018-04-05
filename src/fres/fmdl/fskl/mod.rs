pub mod bone;
pub mod rigid_matrix;
pub mod smooth_matrix;

use self::bone::Bone;
use error::{check_magic_number, UserDataNotEmpty};
use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};
use util::{Importable, IndexGroup, Pointer};

pub struct FSKL {
    pub header: Header,
    pub bones: IndexGroup<Bone>,
}

pub struct Header {
    pub flags: u32,
    pub bone_array_count: u16,
    pub smooth_index_array_count: u16,
    pub rigid_index_array_count: u16,
    pub bone_index_group_offset: Pointer,
    pub bone_array_offset: Pointer,
    pub smooth_index_array_offset: Pointer,
    pub smooth_matrix_array_offset: Pointer,
}

impl Importable for FSKL {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FSKL, Box<Error>> {
        let header = Header::import(reader)?;
        header.bone_index_group_offset.seek_abs_pos(reader)?;
        let bones = IndexGroup::import(reader)?;
        Ok(FSKL { header, bones })
    }
}

impl Importable for Header {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Header, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        check_magic_number(magic_number, [b'F', b'S', b'K', b'L'])?;
        let flags = reader.read_be_to_u32()?;
        let bone_array_count = reader.read_be_to_u16()?;
        let smooth_index_array_count = reader.read_be_to_u16()?;
        let rigid_index_array_count = reader.read_be_to_u16()?;
        reader.seek(SeekFrom::Current(2))?;
        let bone_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let bone_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let smooth_index_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let smooth_matrix_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_pointer = reader.read_be_to_u32()?;
        if user_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: user_pointer,
                data_desc: "User Pointer".to_string(),
            }));
        }
        Ok(Header {
            flags,
            bone_array_count,
            smooth_index_array_count,
            rigid_index_array_count,
            bone_index_group_offset,
            bone_array_offset,
            smooth_index_array_offset,
            smooth_matrix_array_offset,
        })
    }
}
