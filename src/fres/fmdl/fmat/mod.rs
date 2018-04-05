pub mod material_parameter;
pub mod render_info_parameter;
pub mod render_state;
pub mod shader_assign;
pub mod texture_sampler;

use error::{check_magic_number, UserDataNotEmpty};
use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek};
use util::{Importable, Pointer};

pub struct FMAT {
    pub header: Header,
}

pub struct Header {
    pub material_name_offset: Pointer,
    pub material_flags: u32,
    pub section_index: u16,
    pub render_info_parameter_count: u16,
    pub texture_reference_count: u8,
    pub texture_sampler_count: u8,
    pub material_parameter_count: u16,
    pub volatile_parameter_count: u16,
    pub material_parameter_data_length: u16,
    pub raw_parameter_data_length: u16,
    pub user_data_entry_count: u16,
    pub render_info_parameter_index_group_offset: Pointer,
    pub render_state_offset: Pointer,
    pub shader_assign_offset: Pointer,
    pub texture_reference_array_offset: Pointer,
    pub texture_sampler_offset: Pointer,
    pub texture_sampler_index_group_offset: Pointer,
    pub material_parameter_array_offset: Pointer,
    pub material_parameter_index_group_offset: Pointer,
    pub material_parameter_data_offset: Pointer,
    pub user_data_index_group_offset: Pointer,
    pub volatile_flags_data_offset: Pointer,
    pub user_pointer: i32,
}

impl Importable for FMAT {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FMAT, Box<Error>> {
        let header = Header::import(reader)?;
        Ok(FMAT { header })
    }
}

impl Importable for Header {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Header, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        check_magic_number(magic_number, [b'F', b'M', b'A', b'T'])?;
        let material_name_offset = Pointer::read_new_rel_i32_be(reader)?;
        let material_flags = reader.read_be_to_u32()?;
        let section_index = reader.read_be_to_u16()?;
        let render_info_parameter_count = reader.read_be_to_u16()?;
        let texture_reference_count = reader.read_to_u8()?;
        let texture_sampler_count = reader.read_to_u8()?;
        let material_parameter_count = reader.read_be_to_u16()?;
        let volatile_parameter_count = reader.read_be_to_u16()?;
        let material_parameter_data_length = reader.read_be_to_u16()?;
        let raw_parameter_data_length = reader.read_be_to_u16()?;
        let user_data_entry_count = reader.read_be_to_u16()?;
        let render_info_parameter_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let render_state_offset = Pointer::read_new_rel_i32_be(reader)?;
        let shader_assign_offset = Pointer::read_new_rel_i32_be(reader)?;
        let texture_reference_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let texture_sampler_offset = Pointer::read_new_rel_i32_be(reader)?;
        let texture_sampler_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let material_parameter_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let material_parameter_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let material_parameter_data_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_data_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let volatile_flags_data_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_pointer = reader.read_be_to_i32()?;
        if user_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: user_pointer,
                data_desc: "User Pointer".to_string(),
            }));
        }
        Ok(Header {
            material_name_offset,
            material_flags,
            section_index,
            render_info_parameter_count,
            texture_reference_count,
            texture_sampler_count,
            material_parameter_count,
            volatile_parameter_count,
            material_parameter_data_length,
            raw_parameter_data_length,
            user_data_entry_count,
            render_info_parameter_index_group_offset,
            render_state_offset,
            shader_assign_offset,
            texture_reference_array_offset,
            texture_sampler_offset,
            texture_sampler_index_group_offset,
            material_parameter_array_offset,
            material_parameter_index_group_offset,
            material_parameter_data_offset,
            user_data_index_group_offset,
            volatile_flags_data_offset,
            user_pointer,
        })
    }
}
