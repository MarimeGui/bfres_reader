pub mod fmat;
pub mod fshp;
pub mod fskl;
pub mod fvtx;

use self::{fmat::FMAT, fshp::FSHP, fskl::FSKL, fvtx::FVTX};
use error::UserDataNotEmpty;
use error::check_magic_number;
use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek};
use util::{DataArray, Importable, IndexGroup, Pointer};

pub struct FMDL {
    pub header: Header,
    pub fvtx_array: DataArray<FVTX>,
    pub fmat_index_group: IndexGroup<FMAT>,
    pub fskl: FSKL,
    pub fshp_index_group: IndexGroup<FSHP>,
}

pub struct Header {
    pub file_name_offset: Pointer,
    pub file_path_offset: Pointer,
    pub fskl_offset: Pointer,
    pub fvtx_array_offset: Pointer,
    pub fshp_index_group_offset: Pointer,
    pub fmat_index_group_offset: Pointer,
    pub user_data_index_group_offset: Pointer,
    pub fvtx_count: u16,
    pub fshp_count: u16,
    pub fmat_count: u16,
    pub user_data_entry_count: u16,
    pub total_nb_vertices: u32,
    pub user_pointer: u32,
}

impl Importable for FMDL {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FMDL, Box<Error>> {
        let header = Header::import(reader)?;
        header.fvtx_array_offset.seek_abs_pos(reader)?;
        let fvtx_array = DataArray::new(reader, 0x20, u32::from(header.fvtx_count))?;
        header.fmat_index_group_offset.seek_abs_pos(reader)?;
        let fmat_index_group = IndexGroup::import(reader)?;
        header.fskl_offset.seek_abs_pos(reader)?;
        let fskl = FSKL::import(reader)?;
        header.fshp_index_group_offset.seek_abs_pos(reader)?;
        let fshp_index_group = IndexGroup::import(reader)?;
        Ok(FMDL {
            header,
            fvtx_array,
            fmat_index_group,
            fskl,
            fshp_index_group,
        })
    }
}

impl Importable for Header {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Header, Box<Error>> {
        // Magic Number
        let mut magic_number: [u8; 4] = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        check_magic_number(magic_number, [b'F', b'M', b'D', b'L'])?;
        // File Name Offset
        let file_name_offset = Pointer::read_new_rel_i32_be(reader)?;
        // File Path Offset
        let file_path_offset = Pointer::read_new_rel_i32_be(reader)?;
        // FSKL Offset
        let fskl_offset = Pointer::read_new_rel_i32_be(reader)?;
        // FVTX Array Offset
        let fvtx_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        // FSHP Index Group Offset
        let fshp_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        // FMAT Index Group Offset
        let fmat_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        // User Data Index Group Offset
        let user_data_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        // FVTX Count
        let fvtx_count = reader.read_be_to_u16()?;
        // FSHP Count
        let fshp_count = reader.read_be_to_u16()?;
        // FMAT Count
        let fmat_count = reader.read_be_to_u16()?;
        // User Data Entry Count
        let user_data_entry_count = reader.read_be_to_u16()?;
        // Total number of vertices to process
        let total_nb_vertices = reader.read_be_to_u32()?;
        // User Pointer
        let user_pointer = reader.read_be_to_u32()?;
        if user_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: user_pointer,
                data_desc: "User Pointer".to_string(),
            }));
        }
        Ok(Header {
            file_name_offset,
            file_path_offset,
            fskl_offset,
            fvtx_array_offset,
            fshp_index_group_offset,
            fmat_index_group_offset,
            user_data_index_group_offset,
            fvtx_count,
            fshp_count,
            fmat_count,
            user_data_entry_count,
            total_nb_vertices,
            user_pointer,
        })
    }
}
