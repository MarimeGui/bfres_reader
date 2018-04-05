use super::visibility_group::VisibilityGroup;
use error::MissingFSHPLODModelIndexFormat;
use error::MissingFSHPLODModelPrimitiveType;
use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};
use util::{BufferInfo, DataArray, Importable, Pointer};

pub struct LODModel {
    pub primitive_type: PrimitiveType,
    pub index_format: IndexFormat,
    pub nb_points: u32,
    pub nb_visibility_groups: u16,
    pub visibility_group_offset: Pointer,
    pub buffer_info_offset: Pointer,
    pub skip_vertices: u32,
}

pub enum PrimitiveType {
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
    TessellateQuadStrip,
}

pub enum IndexFormat {
    U16LittleEndian = 0,
    U32LittleEndian = 1,
    U16BigEndian = 4,
    U32BigEndian = 9,
}

impl Importable for LODModel {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<LODModel, Box<Error>> {
        let primitive_type = match reader.read_be_to_u32()? {
            0x01 => PrimitiveType::Points,
            0x02 => PrimitiveType::Lines,
            0x03 => PrimitiveType::LineStrip,
            0x04 => PrimitiveType::Triangles,
            0x05 => PrimitiveType::TriangleFan,
            0x06 => PrimitiveType::TriangleStrip,
            0x0A => PrimitiveType::LinesAdjacency,
            0x0B => PrimitiveType::LineStripAdjacency,
            0x0C => PrimitiveType::TrianglesAdjacency,
            0x0D => PrimitiveType::TriangleStripAdjacency,
            0x11 => PrimitiveType::Rectangles,
            0x12 => PrimitiveType::LineLoop,
            0x13 => PrimitiveType::Quads,
            0x14 => PrimitiveType::QuadStrip,
            0x82 => PrimitiveType::TessellateLines,
            0x83 => PrimitiveType::TessellateLineStrip,
            0x84 => PrimitiveType::TessellateTriangles,
            0x86 => PrimitiveType::TessellateTriangleStrip,
            0x93 => PrimitiveType::TessellateQuads,
            0x94 => PrimitiveType::TessellateQuadStrip,
            _ => return Err(Box::new(MissingFSHPLODModelPrimitiveType {})),
        };
        let index_format = match reader.read_be_to_u32()? {
            0 => IndexFormat::U16LittleEndian,
            1 => IndexFormat::U32LittleEndian,
            4 => IndexFormat::U16BigEndian,
            9 => IndexFormat::U32BigEndian,
            _ => return Err(Box::new(MissingFSHPLODModelIndexFormat {})),
        };
        let nb_points = reader.read_be_to_u32()?;
        let nb_visibility_groups = reader.read_be_to_u16()?;
        reader.seek(SeekFrom::Current(2))?;
        let visibility_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let buffer_info_offset = Pointer::read_new_rel_i32_be(reader)?;
        let skip_vertices = reader.read_be_to_u32()?;
        Ok(LODModel {
            primitive_type,
            index_format,
            nb_points,
            nb_visibility_groups,
            visibility_group_offset,
            buffer_info_offset,
            skip_vertices,
        })
    }
}

impl LODModel {
    pub fn get_visibility_groups<R: Read + Seek>(
        &self,
        reader: &mut R,
    ) -> Result<DataArray<VisibilityGroup>, Box<Error>> {
        self.visibility_group_offset.seek_abs_pos(reader)?;
        let array = DataArray::new(reader, 0x18, u32::from(self.nb_visibility_groups))?;
        Ok(array)
    }
    pub fn get_direct_buffer_info<R: Read + Seek>(
        &self,
        reader: &mut R,
    ) -> Result<BufferInfo, Box<Error>> {
        self.buffer_info_offset.seek_abs_pos(reader)?;
        let info = BufferInfo::import(reader)?;
        Ok(info)
    }
}
