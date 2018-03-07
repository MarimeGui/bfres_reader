use ez_io::ReadE;
use IndexGroup;
use DataArray;
use Importable;
use util::Pointer;
use std::io::{Read, Seek, SeekFrom};
use std::error::Error;

pub struct FMDL {
    pub header: FMDLHeader,
    pub fvtx_array: DataArray<FVTX>,
    pub fmat_index_group: IndexGroup<FMAT>,
    pub fskl: FSKL,
    pub fshp_index_group: IndexGroup<FSHP>
}

pub struct FMDLHeader {
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
    pub user_pointer: u32
}

pub struct FVTX {
    pub header: FVTXHeader,
    pub attributes: IndexGroup<FVTXAttributes>,
    pub buffers: DataArray<FVTXBuffer>
}

pub struct FVTXHeader {
    pub attribute_count: u8,
    pub buffer_count: u8,
    pub section_index: u16,
    pub nb_vertices: u32,
    pub vertex_skin_count: u8,
    pub attribute_array_offset: Pointer,
    pub attribute_index_group_offset: Pointer,
    pub buffer_array_offset: Pointer,
    pub user_pointer: u32
}

pub struct FVTXAttributes {
    pub attribute_name_offset: Pointer,
    pub buffer_index: u8,
    pub buffer_offset: u16,
    pub format: u32
}

pub struct FVTXBuffer {
    pub data_pointer: u32,
    pub size: u32,
    pub handle: u32,
    pub stride: u16,
    pub buffering_count: u16,
    pub context_pointer: u32,
    pub data_offset: Pointer
}

pub struct FMAT {
    pub header: FMATHeader
}

pub struct FMATHeader {
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
    pub user_pointer: i32
}

pub struct FMATRenderInfoParameter {

}

pub struct FMATTextureSampler {

}

pub struct FMATMaterialParameter {

}

pub struct FMATRenderState {

}

pub struct FMATShaderAssign {

}

pub struct FSKL {
    pub header: FSKLHeader
}

pub struct FSKLHeader {
    pub flags: u32,
    pub bone_array_count: u16,
    pub smooth_index_array_count: u16,
    pub rigid_index_array_count: u16,
    pub bone_index_group_offset: Pointer,
    pub bone_array_offset: Pointer,
    pub smooth_index_array_offset: Pointer,
    pub smooth_matrix_array_offset: Pointer
}

pub struct FSKLBone {

}

pub struct FSKLSmoothMatrix {

}

pub struct FSKLRigidMatrix {

}

pub struct FSHP {
    pub header: FSHPHeader,
    pub lod_model_array: DataArray<FSHPLODModel>
}

pub struct FSHPHeader {
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

pub struct FSHPLODModel {
    pub primitive_type: u32,
    pub index_format: u32,
    pub nb_points: u32,
    pub nb_visibility_groups: u16,
    pub visibility_group_offset: Pointer,
    pub index_buffer_offset: Pointer,
    pub skip_vertices: u32
}

pub struct FSHPVisibilityGroup {
    pub index_buffer_offset: Pointer,
    pub nb_points: u32
}

pub struct FSHPIndexBuffer {
    pub data_pointer: u32,
    pub size: u32,
    pub handle: u32,
    pub stride: u16,
    pub buffering_count: u16,
    pub context_pointer: u32,
    pub data_offset: Pointer
}

pub struct FSHPVisibilityGroupTree {

}

impl Importable for FMDL {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FMDL, Box<Error>> {
        let header = FMDLHeader::import(reader)?;
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
            fshp_index_group
        })
    }
}

impl Importable for FMDLHeader {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FMDLHeader, Box<Error>> {
        // Magic Number
        let mut magic_number: [u8; 4] = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        assert_eq!(magic_number, [b'F', b'M', b'D', b'L'], "Wrong magic number");
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
        assert_eq!(user_pointer, 0, "User pointer is always 0 in files");
        Ok(FMDLHeader {
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
            user_pointer
        })
    }
}

impl Importable for FVTX {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FVTX, Box<Error>> {
        let header = FVTXHeader::import(reader)?;
        header.attribute_index_group_offset.seek_abs_pos(reader)?;
        let attributes = IndexGroup::import(reader)?;
        header.buffer_array_offset.seek_abs_pos(reader)?;
        let buffers: DataArray<FVTXBuffer> = DataArray::new(reader, 0x18, u32::from(header.buffer_count))?;
        Ok(FVTX {
            header,
            attributes,
            buffers
        })
    }
}

impl Importable for FVTXHeader {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FVTXHeader, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        assert_eq!(magic_number, [b'F', b'V', b'T', b'X'], "Wrong magic number");
        let attribute_count = reader.read_to_u8()?;
        let buffer_count = reader.read_to_u8()?;
        let section_index = reader.read_be_to_u16()?;
        let nb_vertices = reader.read_be_to_u32()?;
        let vertex_skin_count = reader.read_to_u8()?;
        reader.seek(SeekFrom::Current(3))?;
        let attribute_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let attribute_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let buffer_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_pointer: u32 = reader.read_be_to_u32()?;
        assert_eq!(user_pointer, 0, "User pointer is always 0 in files");
        Ok(FVTXHeader {
            attribute_count,
            buffer_count,
            section_index,
            nb_vertices,
            vertex_skin_count,
            attribute_array_offset,
            attribute_index_group_offset,
            buffer_array_offset,
            user_pointer
        })
    }
}

impl Importable for FVTXAttributes {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FVTXAttributes, Box<Error>> {
        let attribute_name_offset = Pointer::read_new_rel_i32_be(reader)?;
        let buffer_index = reader.read_to_u8()?;
        reader.seek(SeekFrom::Current(1))?;
        let buffer_offset = reader.read_be_to_u16()?;
        let format = reader.read_be_to_u32()?;
        Ok(FVTXAttributes {
            attribute_name_offset,
            buffer_index,
            buffer_offset,
            format
        })
    }
}

impl Importable for FVTXBuffer {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FVTXBuffer, Box<Error>> {
        let data_pointer = reader.read_be_to_u32()?;
        assert_eq!(data_pointer, 0, "Data pointer is always 0 in files");
        let size = reader.read_be_to_u32()?;
        let handle = reader.read_be_to_u32()?;
        assert_eq!(handle, 0, "Handle is always 0 in files");
        let stride = reader.read_be_to_u16()?;
        let buffering_count = reader.read_be_to_u16()?;
        let context_pointer = reader.read_be_to_u32()?;
        assert_eq!(context_pointer, 0, "Context pointer is always 0 in files");
        let data_offset = Pointer::read_new_rel_i32_be(reader)?;
        Ok(FVTXBuffer {
            data_pointer,
            size,
            handle,
            stride,
            buffering_count,
            context_pointer,
            data_offset
        })
    }
}

impl Importable for FMAT {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FMAT, Box<Error>> {
        let header = FMATHeader::import(reader)?;
        Ok(FMAT {
            header
        })
    }
}

impl Importable for FMATHeader {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FMATHeader, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        assert_eq!(magic_number, [b'F', b'M', b'A', b'T'], "Wrong magic number");
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
        assert_eq!(user_pointer, 0, "User pointer is always 0 in files");
        Ok(FMATHeader {
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
            user_pointer
        })
    }
}

impl Importable for FSKL {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FSKL, Box<Error>> {
        let header = FSKLHeader::import(reader)?;
        Ok(FSKL {
            header
        })
    }
}

impl Importable for FSKLHeader {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FSKLHeader, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        assert_eq!(magic_number, [b'F', b'S', b'K', b'L'], "Wrong magic number");
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
        assert_eq!(user_pointer, 0, "User pointer is always 0 in files");
        Ok(FSKLHeader {
            flags,
            bone_array_count,
            smooth_index_array_count,
            rigid_index_array_count,
            bone_index_group_offset,
            bone_array_offset,
            smooth_index_array_offset,
            smooth_matrix_array_offset
        })
    }
}

impl Importable for FSHP {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FSHP, Box<Error>> {
        let header = FSHPHeader::import(reader)?;
        header.lod_model_offset.seek_abs_pos(reader)?;
        let lod_model_array = DataArray::new(reader, 0x1C, u32::from(header.lod_model_count))?;
        Ok(FSHP {
            header,
            lod_model_array
        })
    }
}

impl Importable for FSHPHeader {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FSHPHeader, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        assert_eq!(magic_number, [b'F', b'S', b'H', b'P'], "Wrong magic number");
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
        Ok(FSHPHeader {
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
            visibility_group_tree_indices_offset
        })
    }
}

impl Importable for FSHPLODModel {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FSHPLODModel, Box<Error>> {
        let primitive_type = reader.read_be_to_u32()?;
        let index_format = reader.read_be_to_u32()?;
        let nb_points = reader.read_be_to_u32()?;
        let nb_visibility_groups = reader.read_be_to_u16()?;
        reader.seek(SeekFrom::Current(2))?;
        let visibility_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let index_buffer_offset = Pointer::read_new_rel_i32_be(reader)?;
        let skip_vertices = reader.read_be_to_u32()?;
        Ok(FSHPLODModel {
            primitive_type,
            index_format,
            nb_points,
            nb_visibility_groups,
            visibility_group_offset,
            index_buffer_offset,
            skip_vertices
        })
    }
}

impl FSHPLODModel {
    pub fn get_visibility_groups<R: Read + Seek>(&self, reader: &mut R) -> Result<DataArray<FSHPVisibilityGroup>, Box<Error>> {
        self.visibility_group_offset.seek_abs_pos(reader)?;
        let array = DataArray::new(reader, 0x18, u32::from(self.nb_visibility_groups))?;
        Ok(array)
    }
    pub fn get_direct_index_buffer<R: Read + Seek>(&self, reader: &mut R) -> Result<FSHPIndexBuffer, Box<Error>> {
        self.index_buffer_offset.seek_abs_pos(reader)?;
        let group = FSHPIndexBuffer::import(reader)?;
        Ok(group)
    }
}

impl Importable for FSHPVisibilityGroup {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FSHPVisibilityGroup, Box<Error>> {
        let index_buffer_offset = Pointer::read_new_rel_i32_be(reader)?;  // Should be u32
        let nb_points = reader.read_be_to_u32()?;
        Ok(FSHPVisibilityGroup {
            index_buffer_offset,
            nb_points
        })
    }
}

impl FSHPVisibilityGroup {
    pub fn get_index_buffer<R: Read + Seek>(&self, reader: &mut R) -> Result<FSHPIndexBuffer, Box<Error>> {
        self.index_buffer_offset.seek_abs_pos(reader)?;
        let index_buffer = FSHPIndexBuffer::import(reader)?;
        Ok(index_buffer)
    }
}

impl Importable for FSHPIndexBuffer {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FSHPIndexBuffer, Box<Error>> {
        let data_pointer = reader.read_be_to_u32()?;
        assert_eq!(data_pointer, 0, "Data pointer is always 0 in files");
        let size = reader.read_be_to_u32()?;
        let handle = reader.read_be_to_u32()?;
        assert_eq!(handle, 0, "Handle is always 0 in files");
        let stride = reader.read_be_to_u16()?;
        let buffering_count = reader.read_be_to_u16()?;
        let context_pointer = reader.read_be_to_u32()?;
        assert_eq!(context_pointer, 0, "Context pointer is always 0 in files");
        let data_offset = Pointer::read_new_rel_i32_be(reader)?;
        Ok(FSHPIndexBuffer {
            data_pointer,
            size,
            handle,
            stride,
            buffering_count,
            context_pointer,
            data_offset
        })
    }
}