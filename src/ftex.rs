use ez_io::ReadE;
use std::error::Error;
use std::io::Read;
use Importable;
use error::WrongMagicNumber;

pub struct FTEX {
    pub header: FTEXHeader
}

pub struct FTEXHeader {
    pub magic_number: [u8; 4],
    pub dimension: u32,
    pub texture_width: u32,
    pub texture_height: u32,
    pub texture_depth: u32,
    pub nb_mipmaps: u32,
    pub texture_format: u32,
    pub aa_mode: u32,
    pub usage: u32,
    pub data_length: u32,
    pub data_pointer: u32,
    pub mipmaps_data_length: u32,
    pub mipmaps_pointer: u32,
    pub tile_mode: u32,
    pub swizzle_value: u32,
    pub alignment: u32,
    pub pitch: u32,
    pub mipmap_offsets: [u32; 13],
    pub first_mipmap: u32,
    pub nb_mipmaps2: u32,
    pub first_slice: u32,
    pub component_selector: [u8; 4],
    pub texture_registers: [u32; 5],
    pub texture_handle: u32,
    pub array_length: u32,
    pub file_name_offset: i32,
    pub file_path_offset: i32,
    pub data_offset: i32,
    pub mipmap_offset: i32,
    pub user_data_index_group_offset: i32,
    pub user_data_entry_count: u16
}

impl Importable for FTEXHeader {
    fn import<R: Read>(reader: &mut R) -> Result<FTEXHeader, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        if magic_number != [b'F', b'T', b'E', b'X'] {
            return Err(Box::new(WrongMagicNumber{}))
        }
        let dimension = reader.read_be_to_u32()?;
        let texture_width = reader.read_be_to_u32()?;
        let texture_height = reader.read_be_to_u32()?;
        let texture_depth = reader.read_be_to_u32()?;
        let nb_mipmaps = reader.read_be_to_u32()?;
        let texture_format = reader.read_be_to_u32()?;
        let aa_mode = reader.read_be_to_u32()?;
        let usage = reader.read_be_to_u32()?;
        let data_length = reader.read_be_to_u32()?;
        let data_pointer = reader.read_be_to_u32()?;
        let mipmaps_data_length = reader.read_be_to_u32()?;
        let mipmaps_pointer = reader.read_be_to_u32()?;
        let tile_mode = reader.read_be_to_u32()?;
        let swizzle_value = reader.read_be_to_u32()?;
        let alignment = reader.read_be_to_u32()?;
        let pitch = reader.read_be_to_u32()?;
        let mut mipmap_offsets: [u32; 13] = [0u32; 13];
        for data in &mut mipmap_offsets {
            *data = reader.read_be_to_u32()?;
        }
        let first_mipmap = reader.read_be_to_u32()?;
        let nb_mipmaps2 = reader.read_be_to_u32()?;
        let first_slice = reader.read_be_to_u32()?;
        let mut component_selector: [u8; 4] = [0u8; 4];
        reader.read_exact(&mut component_selector)?;
        let mut texture_registers: [u32; 5] = [0u32; 5];
        for data in &mut texture_registers {
            *data = reader.read_be_to_u32()?;
        }
        let texture_handle = reader.read_be_to_u32()?;
        let array_length = reader.read_be_to_u32()?;
        let file_name_offset = reader.read_be_to_i32()?;
        let file_path_offset = reader.read_be_to_i32()?;
        let data_offset = reader.read_be_to_i32()?;
        let mipmap_offset = reader.read_be_to_i32()?;
        let user_data_index_group_offset = reader.read_be_to_i32()?;
        let user_data_entry_count = reader.read_be_to_u16()?;
        Ok(FTEXHeader {
            magic_number,
            dimension,
            texture_width,
            texture_height,
            texture_depth,
            nb_mipmaps,
            texture_format,
            aa_mode,
            usage,
            data_length,
            data_pointer,
            mipmaps_data_length,
            mipmaps_pointer,
            tile_mode,
            swizzle_value,
            alignment,
            pitch,
            mipmap_offsets,
            first_mipmap,
            nb_mipmaps2,
            first_slice,
            component_selector,
            texture_registers,
            texture_handle,
            array_length,
            file_name_offset,
            file_path_offset,
            data_offset,
            mipmap_offset,
            user_data_index_group_offset,
            user_data_entry_count
        })
    }
}