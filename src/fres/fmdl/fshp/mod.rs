pub mod lod_model;
pub mod visibility_group;

use self::lod_model::LODModel;
use error::check_magic_number;
use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek};
use util::{DataArray, Importable, Pointer};

pub struct FSHP {
    pub header: Header,
    pub lod_model_array: DataArray<LODModel>,
}

pub struct Header {
    pub polygon_name_offset: Pointer,
    pub flags: u32,
    pub section_index: u16,
    pub fmat_index: u16,
    pub fskl_index: u16,
    pub fvtx_index: u16,
    pub fskl_bone_skin_index: u16,
    pub vertex_skin_count: u8,
    pub lod_model_count: u8,
    pub key_shape_count: u8,
    pub target_attribute_count: u8,
    pub visibility_group_tree_node_count: u16,
    pub bounding_box_radius: u32,
    pub fvtx_offset: Pointer,
    pub lod_model_offset: Pointer,
    pub fskl_index_array_offset: Pointer,
    pub key_shape_index_group_offset: Pointer,
    pub visibility_group_tree_nodes_offset: Pointer,
    pub visibility_group_tree_ranges_offset: Pointer,
    pub visibility_group_tree_indices_offset: Pointer,
}

impl Importable for FSHP {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FSHP, Box<Error>> {
        let header = Header::import(reader)?;
        header.lod_model_offset.seek_abs_pos(reader)?;
        let lod_model_array = DataArray::new(reader, 0x1C, u32::from(header.lod_model_count))?;
        Ok(FSHP {
            header,
            lod_model_array,
        })
    }
}

impl Importable for Header {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Header, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        check_magic_number(magic_number, [b'F', b'S', b'H', b'P'])?;
        let polygon_name_offset = Pointer::read_new_rel_i32_be(reader)?;
        let flags = reader.read_be_to_u32()?;
        let section_index = reader.read_be_to_u16()?;
        let fmat_index = reader.read_be_to_u16()?;
        let fskl_index = reader.read_be_to_u16()?;
        let fvtx_index = reader.read_be_to_u16()?;
        let fskl_bone_skin_index = reader.read_be_to_u16()?;
        let vertex_skin_count = reader.read_to_u8()?;
        let lod_model_count = reader.read_to_u8()?;
        let key_shape_count = reader.read_to_u8()?;
        let target_attribute_count = reader.read_to_u8()?;
        let visibility_group_tree_node_count = reader.read_be_to_u16()?;
        let bounding_box_radius = reader.read_be_to_u32()?;
        let fvtx_offset = Pointer::read_new_rel_i32_be(reader)?;
        let lod_model_offset = Pointer::read_new_rel_i32_be(reader)?;
        let fskl_index_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let key_shape_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let visibility_group_tree_nodes_offset = Pointer::read_new_rel_i32_be(reader)?;
        let visibility_group_tree_ranges_offset = Pointer::read_new_rel_i32_be(reader)?;
        let visibility_group_tree_indices_offset = Pointer::read_new_rel_i32_be(reader)?;
        // let user_pointer = reader.read_be_to_u32()?;
        // assert_eq!(user_pointer, 0, "User pointer is always 0 in files");  It seems as like this one is not 0
        Ok(Header {
            polygon_name_offset,
            flags,
            section_index,
            fmat_index,
            fskl_index,
            fvtx_index,
            fskl_bone_skin_index,
            vertex_skin_count,
            lod_model_count,
            key_shape_count,
            target_attribute_count,
            visibility_group_tree_node_count,
            bounding_box_radius,
            fvtx_offset,
            lod_model_offset,
            fskl_index_array_offset,
            key_shape_index_group_offset,
            visibility_group_tree_nodes_offset,
            visibility_group_tree_ranges_offset,
            visibility_group_tree_indices_offset,
        })
    }
}
