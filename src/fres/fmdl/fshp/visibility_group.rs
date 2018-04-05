use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek};
use util::{BufferInfo, Importable, Pointer};

pub struct VisibilityGroup {
    pub buffer_info_offset: Pointer,
    pub nb_points: u32,
}

pub struct Tree {}

impl Importable for VisibilityGroup {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<VisibilityGroup, Box<Error>> {
        let buffer_info_offset = Pointer::read_new_rel_i32_be(reader)?; // Should be u32
        let nb_points = reader.read_be_to_u32()?;
        Ok(VisibilityGroup {
            buffer_info_offset,
            nb_points,
        })
    }
}

impl VisibilityGroup {
    pub fn get_index_buffer<R: Read + Seek>(
        &self,
        reader: &mut R,
    ) -> Result<BufferInfo, Box<Error>> {
        self.buffer_info_offset.seek_abs_pos(reader)?;
        let buffer_info = BufferInfo::import(reader)?;
        Ok(buffer_info)
    }
}
