use ez_io::ReadE;
use IndexGroup;
use Importable;
use util::Pointer;
use error::WrongMagicNumber;
use std::io::{Read, Seek};
use std::error::Error;

pub struct FMDL {
    pub header: FMDLHeader,
    pub fvtx_array: Vec<FVTX>,
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
    pub header: FVTXHeader
    // pub attributes: Vec<FVTXAttributes>, I will do that later
    // pub buffers: Vec<FVTXBuffers>
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

pub struct FVTXBuffers {
    pub data_pointer: u32,
    pub size: u32,
    pub handle: u32,
    pub stride: u16,
    pub buffering_count: u16,
    pub context_pointer: u32,
    pub data_offset: Pointer
}

pub struct FMAT {

}

pub struct FMATHeader {

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

}

pub struct FSKLHeader {

}

pub struct FSKLBone {

}

pub struct FSKLSmoothMatrix {

}

pub struct FSKLRigidMatrix {

}

pub struct FSHP {

}

pub struct FSHPHeader {

}

pub struct FSHPLODModel {

}

pub struct FSHPVisibilityGroup {

}

pub struct FSHPIndexBuffer {

}

pub struct FSHPVisibilityGroupTree {

}

impl Importable for FMDL {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FMDL, Box<Error>> {
        let header = FMDLHeader::import(reader)?;
        header.fvtx_array_offset.seek_abs_pos(reader)?;
        let mut fvtx_array: Vec<FVTX> = Vec::with_capacity(header.fvtx_count as usize);
        for _ in 0..header.fvtx_count {
            fvtx_array.push(FVTX::import(reader)?);
        }
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
        if magic_number != [b'F', b'M', b'D', b'L'] {
            return Err(Box::new(WrongMagicNumber {}));
        }
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
        Ok(FVTX {
            header
        })
    }
}

impl Importable for FVTXHeader {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FVTXHeader, Box<Error>> {
        let attribute_count = reader.read_to_u8()?;
        let buffer_count = reader.read_to_u8()?;
        let section_index = reader.read_be_to_u16()?;
        let nb_vertices = reader.read_be_to_u32()?;
        let vertex_skin_count = reader.read_to_u8()?;
        let attribute_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let attribute_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let buffer_array_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_pointer: u32 = reader.read_be_to_u32()?;
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

impl Importable for FMAT {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FMAT, Box<Error>> {
        Ok(FMAT {})
    }
}

impl Importable for FSKL {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FSKL, Box<Error>> {
        Ok(FSKL {})
    }
}

impl Importable for FSHP {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FSHP, Box<Error>> {
        Ok(FSHP {})
    }
}