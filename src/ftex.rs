use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek};
use Importable;
use util::Pointer;
use error::WrongMagicNumber;

pub struct FTEX {
    pub header: FTEXHeader
}

pub struct FTEXHeader {
    pub dimension: u32,
    pub texture_width: u32,
    pub texture_height: u32,
    pub texture_depth: u32,
    pub nb_mipmaps: u32,
    pub texture_format: u32,
    pub aa_mode: u32,
    pub usage: u32,
    pub data_length: u32,
    pub mipmaps_data_length: u32,
    pub tile_mode: u32,
    pub swizzle_value: u32,
    pub alignment: u32,
    pub pitch: u32,
    pub mipmap_offsets: [u32; 13],
    pub first_mipmap: u32,
    pub nb_mipmaps2: u32,
    pub component_selector: [u8; 4],
    pub texture_registers: [u32; 5],
    pub array_length: u32,
    pub file_name_offset: Pointer,
    pub file_path_offset: Pointer,
    pub data_offset: Pointer,
    pub mipmap_offset: Pointer,
    pub user_data_index_group_offset: Pointer,
    pub user_data_entry_count: u16
}

impl Importable for FTEX {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEX, Box<Error>> {
        let header = FTEXHeader::import(reader)?;
        Ok(FTEX {
            header
        })
    }
}

impl Importable for FTEXHeader {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEXHeader, Box<Error>> {
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
        assert_eq!(data_pointer, 0, "Data pointer is always 0 in files");
        let mipmaps_data_length = reader.read_be_to_u32()?;
        let mipmaps_pointer = reader.read_be_to_u32()?;
        assert_eq!(mipmaps_pointer, 0, "Mipmaps pointer is always 0 in files");
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
        assert_eq!(nb_mipmaps, nb_mipmaps2, "The two number of mipmaps are not equal");
        let first_slice = reader.read_be_to_u32()?;
        assert_eq!(first_slice, 0, "First slice is always 0");
        let nb_slices = reader.read_be_to_u32()?;
        assert_eq!(nb_slices, 1, "Number of slices is always 1");
        let mut component_selector: [u8; 4] = [0u8; 4];
        reader.read_exact(&mut component_selector)?;
        let mut texture_registers: [u32; 5] = [0u32; 5];
        for data in &mut texture_registers {
            *data = reader.read_be_to_u32()?;
        }
        let texture_handle = reader.read_be_to_u32()?;
        assert_eq!(texture_handle, 0, "Texture handle is always 0 in files");
        let array_length = reader.read_be_to_u32()?;
        let file_name_offset = Pointer::read_new_rel_i32_be(reader)?;
        let file_path_offset = Pointer::read_new_rel_i32_be(reader)?;
        let data_offset = Pointer::read_new_rel_i32_be(reader)?;
        let mipmap_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_data_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_data_entry_count = reader.read_be_to_u16()?;
        Ok(FTEXHeader {
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
            nb_mipmaps2,
            component_selector,
            texture_registers,
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