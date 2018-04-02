use ez_io::ReadE;
use IndexGroup;
use DataArray;
use Importable;
use util::Pointer;
use error::{MissingFVTXAttributeFormat, MissingFSHPLODModelPrimitiveType, MissingFSHPLODModelIndexFormat, UserDataNotEmpty};
use std::io::{Read, Seek, SeekFrom};
use std::fmt::{Display, Formatter, Result as FMTResult};
use std::error::Error;
use error::check_magic_number;

pub struct BufferInfo {
    pub size: u32,
    pub stride: u16,
    pub buffering_count: u16,
    pub data_offset: Pointer
}

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
    pub attributes_index_group: IndexGroup<FVTXAttributes>,
    pub buffer_info_array: DataArray<BufferInfo>
}

pub struct FVTXHeader {
    pub attribute_count: u8,
    pub buffer_info_count: u8,
    pub section_index: u16,
    pub nb_vertices: u32,
    pub vertex_skin_count: u8,
    pub attribute_array_offset: Pointer,
    pub attribute_index_group_offset: Pointer,
    pub buffer_info_array_offset: Pointer,
    pub user_pointer: u32
}

pub struct FVTXAttributes {
    pub attribute_name_offset: Pointer,
    pub buffer_info_index: u8,
    pub buffer_offset: u16,  // Unsure if points to Info or actual buffer
    pub format: FVTXAttributesFormats
}

pub enum FVTXAttributesFormats {
    U8ToF32 = 0x000,
    TwoU8ToTwoF32 = 0x004,
    TwoU16ToTwoF32 = 0x007,
    FourU8ToFourF32 = 0x00A,
    U8ToU32 = 0x100,
    TwoU8ToTwoU32 = 0x104,
    FourU8ToFourU32 = 0x10A,
    I8ToF32 = 0x200,
    TwoI8ToF32 = 0x204,
    TwoI16ToTwoF32 = 0x207,
    FourI8ToFourF32 = 0x20A,
    ThreeI10toThreeF32 = 0x20B,
    I8 = 0x300,
    TwoI8 = 0x304,
    FourI8 = 0x30A,
    F32 = 0x806,
    TwoF16ToTwoF32 = 0x808,
    TwoF32 = 0x80D,
    FourF16ToFourF32 = 0x80F,
    ThreeF32 = 0x811,
    FourF32 = 0x813
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
    pub primitive_type: FSHPLODModelPrimitiveType,
    pub index_format: FSHPLODModelIndexFormat,
    pub nb_points: u32,
    pub nb_visibility_groups: u16,
    pub visibility_group_offset: Pointer,
    pub buffer_info_offset: Pointer,
    pub skip_vertices: u32
}

pub enum FSHPLODModelPrimitiveType {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleFan,
    TriangleStrip,
    LinesAdjacency,
    LineStripAdjacency,
    TrianglesAdjacency,
    TriangleStripAdjacency,
    Rectangles,
    LineLoop,
    Quads,
    QuadStrip,
    TessellateLines,
    TessellateLineStrip,
    TessellateTriangles,
    TessellateTriangleStrip,
    TessellateQuads,
    TessellateQuadStrip
}

pub enum FSHPLODModelIndexFormat {
    U16LittleEndian = 0,
    U32LittleEndian = 1,
    U16BigEndian = 4,
    U32BigEndian = 9
}

pub struct FSHPVisibilityGroup {
    pub buffer_info_offset: Pointer,
    pub nb_points: u32
}

pub struct FSHPVisibilityGroupTree {

}

impl Importable for BufferInfo {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<BufferInfo, Box<Error>> {
        let data_pointer = reader.read_be_to_u32()?;
        if data_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {data: data_pointer, data_desc: "Data Pointer".to_string()}))
        }
        let size = reader.read_be_to_u32()?;
        let handle = reader.read_be_to_u32()?;
        if handle != 0 {
            return Err(Box::new(UserDataNotEmpty {data: handle, data_desc: "Handle".to_string()}))
        }
        let stride = reader.read_be_to_u16()?;
        let buffering_count = reader.read_be_to_u16()?;
        let context_pointer = reader.read_be_to_u32()?;
        if context_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {data: context_pointer, data_desc: "Context Pointer".to_string()}))
        }
        let data_offset = Pointer::read_new_rel_i32_be(reader)?;
        Ok(BufferInfo {
            size,
            stride,
            buffering_count,
            data_offset
        })
    }
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
            return Err(Box::new(UserDataNotEmpty {data: user_pointer, data_desc: "User Pointer".to_string()}))
        }
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
        header.buffer_info_array_offset.seek_abs_pos(reader)?;
        let buffer_info_array = DataArray::new(reader, 0x18, u32::from(header.buffer_info_count))?;
        Ok(FVTX {
            header,
            attributes_index_group: attributes,
            buffer_info_array
        })
    }
}

impl Importable for FVTXHeader {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FVTXHeader, Box<Error>> {
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
            return Err(Box::new(UserDataNotEmpty {data: user_pointer, data_desc: "User Pointer".to_string()}))
        }
        Ok(FVTXHeader {
            attribute_count,
            buffer_info_count,
            section_index,
            nb_vertices,
            vertex_skin_count,
            attribute_array_offset,
            attribute_index_group_offset,
            buffer_info_array_offset,
            user_pointer
        })
    }
}

impl Importable for FVTXAttributes {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FVTXAttributes, Box<Error>> {
        let attribute_name_offset = Pointer::read_new_rel_i32_be(reader)?;
        let buffer_info_index = reader.read_to_u8()?;
        reader.seek(SeekFrom::Current(1))?;
        let buffer_offset = reader.read_be_to_u16()?;
        let format = match reader.read_be_to_u32()? {
            0x0000 => FVTXAttributesFormats::U8ToF32,
            0x0004 => FVTXAttributesFormats::TwoU8ToTwoF32,
            0x0007 => FVTXAttributesFormats::TwoU16ToTwoF32,
            0x000A => FVTXAttributesFormats::FourU8ToFourF32,
            0x0100 => FVTXAttributesFormats::U8ToU32,
            0x0104 => FVTXAttributesFormats::TwoU8ToTwoU32,
            0x010A => FVTXAttributesFormats::FourU8ToFourU32,
            0x0200 => FVTXAttributesFormats::I8ToF32,
            0x0204 => FVTXAttributesFormats::TwoI8ToF32,
            0x0207 => FVTXAttributesFormats::TwoI16ToTwoF32,
            0x020A => FVTXAttributesFormats::FourI8ToFourF32,
            0x020B => FVTXAttributesFormats::ThreeI10toThreeF32,
            0x0300 => FVTXAttributesFormats::I8,
            0x0304 => FVTXAttributesFormats::TwoI8,
            0x030A => FVTXAttributesFormats::FourI8,
            0x0806 => FVTXAttributesFormats::F32,
            0x0808 => FVTXAttributesFormats::TwoF16ToTwoF32,
            0x080D => FVTXAttributesFormats::TwoF32,
            0x080F => FVTXAttributesFormats::FourF16ToFourF32,
            0x0811 => FVTXAttributesFormats::ThreeF32,
            0x0813 => FVTXAttributesFormats::FourF32,
            _ => return Err(Box::new(MissingFVTXAttributeFormat {}))
        };
        Ok(FVTXAttributes {
            attribute_name_offset,
            buffer_info_index,
            buffer_offset,
            format
        })
    }
}

impl Display for FVTXAttributesFormats {
    fn fmt(&self, f: &mut Formatter) -> FMTResult {
        let text = match *self {
            FVTXAttributesFormats::U8ToF32 => "One u8 to one F32",
            FVTXAttributesFormats::TwoU8ToTwoF32 => "Two u8 to two f32",
            FVTXAttributesFormats::TwoU16ToTwoF32 => "Two u16 to two f32",
            FVTXAttributesFormats::FourU8ToFourF32 => "Four u8 to four f32",
            FVTXAttributesFormats::U8ToU32 => "One u8 to one u32",
            FVTXAttributesFormats::TwoU8ToTwoU32 => "Two u8 to two u32",
            FVTXAttributesFormats::FourU8ToFourU32 => "Four u8 to four u32",
            FVTXAttributesFormats::I8ToF32 => "One i8 to one f32",
            FVTXAttributesFormats::TwoI8ToF32 => "Two i8 to one f32",
            FVTXAttributesFormats::TwoI16ToTwoF32 => "Two i16 to two f32",
            FVTXAttributesFormats::FourI8ToFourF32 => "Four i8 to four f32",
            FVTXAttributesFormats::ThreeI10toThreeF32 => "Three i10 to three f32",
            FVTXAttributesFormats::I8 => "One i8",
            FVTXAttributesFormats::TwoI8 => "Two i8",
            FVTXAttributesFormats::FourI8 => "Four i8",
            FVTXAttributesFormats::F32 => "One f32",
            FVTXAttributesFormats::TwoF16ToTwoF32 => "Two f16 to two f32",
            FVTXAttributesFormats::TwoF32 => "Two f32",
            FVTXAttributesFormats::FourF16ToFourF32 => "Four f16 to four f32",
            FVTXAttributesFormats::ThreeF32 => "Three f32",
            FVTXAttributesFormats::FourF32 => "Four f32",
        };
        write!(f, "{}", text)
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
            return Err(Box::new(UserDataNotEmpty {data: user_pointer, data_desc: "User Pointer".to_string()}))
        }
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
            return Err(Box::new(UserDataNotEmpty {data: user_pointer, data_desc: "User Pointer".to_string()}))
        }
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
        let primitive_type = match reader.read_be_to_u32()? {
            0x01 => FSHPLODModelPrimitiveType::Points,
            0x02 => FSHPLODModelPrimitiveType::Lines,
            0x03 => FSHPLODModelPrimitiveType::LineStrip,
            0x04 => FSHPLODModelPrimitiveType::Triangles,
            0x05 => FSHPLODModelPrimitiveType::TriangleFan,
            0x06 => FSHPLODModelPrimitiveType::TriangleStrip,
            0x0A => FSHPLODModelPrimitiveType::LinesAdjacency,
            0x0B => FSHPLODModelPrimitiveType::LineStripAdjacency,
            0x0C => FSHPLODModelPrimitiveType::TrianglesAdjacency,
            0x0D => FSHPLODModelPrimitiveType::TriangleStripAdjacency,
            0x11 => FSHPLODModelPrimitiveType::Rectangles,
            0x12 => FSHPLODModelPrimitiveType::LineLoop,
            0x13 => FSHPLODModelPrimitiveType::Quads,
            0x14 => FSHPLODModelPrimitiveType::QuadStrip,
            0x82 => FSHPLODModelPrimitiveType::TessellateLines,
            0x83 => FSHPLODModelPrimitiveType::TessellateLineStrip,
            0x84 => FSHPLODModelPrimitiveType::TessellateTriangles,
            0x86 => FSHPLODModelPrimitiveType::TessellateTriangleStrip,
            0x93 => FSHPLODModelPrimitiveType::TessellateQuads,
            0x94 => FSHPLODModelPrimitiveType::TessellateQuadStrip,
            _ => return Err(Box::new(MissingFSHPLODModelPrimitiveType {}))
        };
        let index_format = match reader.read_be_to_u32()? {
            0 => FSHPLODModelIndexFormat::U16LittleEndian,
            1 => FSHPLODModelIndexFormat::U32LittleEndian,
            4 => FSHPLODModelIndexFormat::U16BigEndian,
            9 => FSHPLODModelIndexFormat::U32BigEndian,
            _ => return Err(Box::new(MissingFSHPLODModelIndexFormat {}))
        };
        let nb_points = reader.read_be_to_u32()?;
        let nb_visibility_groups = reader.read_be_to_u16()?;
        reader.seek(SeekFrom::Current(2))?;
        let visibility_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let buffer_info_offset = Pointer::read_new_rel_i32_be(reader)?;
        let skip_vertices = reader.read_be_to_u32()?;
        Ok(FSHPLODModel {
            primitive_type,
            index_format,
            nb_points,
            nb_visibility_groups,
            visibility_group_offset,
            buffer_info_offset,
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
    pub fn get_direct_buffer_info<R: Read + Seek>(&self, reader: &mut R) -> Result<BufferInfo, Box<Error>> {
        self.buffer_info_offset.seek_abs_pos(reader)?;
        let info = BufferInfo::import(reader)?;
        Ok(info)
    }
}

impl Importable for FSHPVisibilityGroup {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FSHPVisibilityGroup, Box<Error>> {
        let buffer_info_offset = Pointer::read_new_rel_i32_be(reader)?;  // Should be u32
        let nb_points = reader.read_be_to_u32()?;
        Ok(FSHPVisibilityGroup {
            buffer_info_offset,
            nb_points
        })
    }
}

impl FSHPVisibilityGroup {
    pub fn get_index_buffer<R: Read + Seek>(&self, reader: &mut R) -> Result<BufferInfo, Box<Error>> {
        self.buffer_info_offset.seek_abs_pos(reader)?;
        let buffer_info = BufferInfo::import(reader)?;
        Ok(buffer_info)
    }
}