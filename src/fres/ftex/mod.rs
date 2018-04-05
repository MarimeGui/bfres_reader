pub mod aa_mode;
pub mod component_selector;
pub mod dimension;
pub mod format;
pub mod tile_mode;
pub mod usage;

use self::{aa_mode::AAMode, component_selector::ComponentSelector, dimension::Dimension,
           format::Format, tile_mode::TileMode, usage::Usage};
use error::{check_magic_number, UserDataNotEmpty};
use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek};
use util::Importable;
use util::Pointer;

pub struct FTEX {
    pub header: Header,
}

pub struct Header {
    pub dimension: Dimension,
    pub texture_width: u32,
    pub texture_height: u32,
    pub texture_depth: u32,
    pub nb_mipmaps: u32,
    pub texture_format: Format,
    pub aa_mode: AAMode,
    pub usage: Usage,
    pub data_length: u32,
    pub mipmaps_data_length: u32,
    pub tile_mode: TileMode,
    pub swizzle_value: u32,
    pub alignment: u32,
    pub pitch: u32,
    pub mipmap_offsets: [u32; 13],
    pub first_mipmap: u32,
    pub nb_slices: u32,
    pub component_selector: ComponentSelector,
    pub texture_registers: [u32; 5],
    pub array_length: u32,
    pub file_name_offset: Pointer,
    pub file_path_offset: Pointer,
    pub data_offset: Pointer,
    pub mipmap_offset: Pointer,
    pub user_data_index_group_offset: Pointer,
    pub user_data_entry_count: u16,
}

impl Importable for FTEX {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEX, Box<Error>> {
        let header = Header::import(reader)?;
        Ok(FTEX { header })
    }
}

impl Importable for Header {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Header, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        check_magic_number(magic_number, [b'F', b'T', b'E', b'X'])?;
        let dimension = Dimension::import(reader)?;
        let texture_width = reader.read_be_to_u32()?;
        let texture_height = reader.read_be_to_u32()?;
        let texture_depth = reader.read_be_to_u32()?;
        let nb_mipmaps = reader.read_be_to_u32()?;
        let texture_format = Format::import(reader)?;
        let aa_mode = AAMode::import(reader)?;
        let usage = Usage::import(reader)?;
        let data_length = reader.read_be_to_u32()?;
        let data_pointer = reader.read_be_to_u32()?;
        if data_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: data_pointer,
                data_desc: "Data pointer".to_string(),
            }));
        }
        let mipmaps_data_length = reader.read_be_to_u32()?;
        let mipmaps_pointer = reader.read_be_to_u32()?;
        if mipmaps_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: mipmaps_pointer,
                data_desc: "Mipmaps Pointer".to_string(),
            }));
        }
        let tile_mode = TileMode::import(reader)?;
        let swizzle_value = reader.read_be_to_u32()?;
        let alignment = reader.read_be_to_u32()?;
        let pitch = reader.read_be_to_u32()?;
        let mut mipmap_offsets: [u32; 13] = [0u32; 13];
        for data in &mut mipmap_offsets {
            *data = reader.read_be_to_u32()?;
        }
        let first_mipmap = reader.read_be_to_u32()?;
        let nb_mipmaps2 = reader.read_be_to_u32()?;
        assert_eq!(
            nb_mipmaps, nb_mipmaps2,
            "The two number of mipmaps are not equal"
        );
        let first_slice = reader.read_be_to_u32()?;
        assert_eq!(first_slice, 0, "First slice is always 0");
        let nb_slices = reader.read_be_to_u32()?;
        let component_selector = ComponentSelector::import(reader)?;
        let mut texture_registers: [u32; 5] = [0u32; 5];
        for data in &mut texture_registers {
            *data = reader.read_be_to_u32()?;
        }
        let texture_handle = reader.read_be_to_u32()?;
        if texture_handle != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: texture_handle,
                data_desc: "Texture Handle".to_string(),
            }));
        }
        let array_length = reader.read_be_to_u32()?;
        let file_name_offset = Pointer::read_new_rel_i32_be(reader)?;
        let file_path_offset = Pointer::read_new_rel_i32_be(reader)?;
        let data_offset = Pointer::read_new_rel_i32_be(reader)?;
        let mipmap_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_data_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_data_entry_count = reader.read_be_to_u16()?;
        Ok(Header {
            dimension,
            texture_width,
            texture_height,
            texture_depth,
            nb_mipmaps,
            texture_format,
            aa_mode,
            usage,
            data_length,
            mipmaps_data_length,
            tile_mode,
            swizzle_value,
            alignment,
            pitch,
            mipmap_offsets,
            first_mipmap,
            nb_slices,
            component_selector,
            texture_registers,
            array_length,
            file_name_offset,
            file_path_offset,
            data_offset,
            mipmap_offset,
            user_data_index_group_offset,
            user_data_entry_count,
        })
    }
}
