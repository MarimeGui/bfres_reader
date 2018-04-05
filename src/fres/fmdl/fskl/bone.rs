use error::UnrecognizedFSKLBoneFlagProjectionMode;
use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek};
use util::{Importable, Pointer};

pub struct Bone {
    pub name_offset: Pointer,
    pub bone_index: u16,
    pub parent_index: u16,
    pub smooth_matrix_index: i16,
    pub rigid_matrix_index: i16,
    pub billboard_index: i16,
    pub user_data_entry_count: u16,
    pub flags: Flags,
    pub scale_vectors: [f32; 3],
    pub rotation_vectors: [f32; 4],
    pub translation_vectors: [f32; 3],
    pub user_data_index_group_offset: Pointer,
}

pub struct Flags {
    pub visible: bool,
    pub rotation: RotationMode,
    pub projection_mode: BillboardBonesProjectionMode,
    pub transformation_flags: TransformationFlags,
    pub bone_hierarchy_flags: BoneHierarchyFlags,
}

pub enum RotationMode {
    XYZEuler = 1,
    Quaternion = 0,
}

pub enum BillboardBonesProjectionMode {
    None = 0,
    Child = 1,
    WorldViewVector = 2,
    WorldViewPoint = 3,
    ScreenViewVector = 4,
    ScreenViewPoint = 5,
    YAxisViewVector = 6,
    YAxisViewPoint = 7,
}

pub struct TransformationFlags {
    pub segment_scale_compensation: bool,
    pub scale_uniformly: bool,
    pub scale_volume_by_1: bool,
    pub no_rotation: bool,
    pub no_translation: bool,
}

pub struct BoneHierarchyFlags {
    pub scale_uniformly: bool,
    pub scale_volume_by_1: bool,
    pub no_rotation: bool,
    pub no_translation: bool,
}

impl Importable for Bone {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<Error>> {
        let name_offset = Pointer::read_new_rel_i32_be(reader)?;
        let bone_index = reader.read_be_to_u16()?;
        let parent_index = reader.read_be_to_u16()?;
        let smooth_matrix_index = reader.read_be_to_i16()?;
        let rigid_matrix_index = reader.read_be_to_i16()?;
        let billboard_index = reader.read_be_to_i16()?;
        let user_data_entry_count = reader.read_be_to_u16()?;
        let flags = Flags::import(reader)?;
        let scale_vectors = [
            reader.read_be_to_f32()?,
            reader.read_be_to_f32()?,
            reader.read_be_to_f32()?,
        ];
        let rotation_vectors = [
            reader.read_be_to_f32()?,
            reader.read_be_to_f32()?,
            reader.read_be_to_f32()?,
            reader.read_be_to_f32()?,
        ];
        let translation_vectors = [
            reader.read_be_to_f32()?,
            reader.read_be_to_f32()?,
            reader.read_be_to_f32()?,
        ];
        let user_data_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        Ok(Bone {
            name_offset,
            bone_index,
            parent_index,
            smooth_matrix_index,
            rigid_matrix_index,
            billboard_index,
            user_data_entry_count,
            flags,
            scale_vectors,
            rotation_vectors,
            translation_vectors,
            user_data_index_group_offset,
        })
    }
}

impl Importable for Flags {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<Error>> {
        let raw_bits = reader.read_be_to_u32()?;
        let visible = match raw_bits & 0b00000000_00000000_00000000_00000001 {
            0 => false,
            _ => true,
        };
        let rotation = match (raw_bits & 0b00000000_00000000_00010000_00000000) >> 12 {
            0 => RotationMode::Quaternion,
            _ => RotationMode::XYZEuler,
        };
        let projection_mode = match (raw_bits & 0b00000000_00000111_00000000_00000000) >> 16 {
            0 => BillboardBonesProjectionMode::None,
            1 => BillboardBonesProjectionMode::Child,
            2 => BillboardBonesProjectionMode::WorldViewVector,
            3 => BillboardBonesProjectionMode::WorldViewPoint,
            4 => BillboardBonesProjectionMode::ScreenViewVector,
            5 => BillboardBonesProjectionMode::ScreenViewPoint,
            6 => BillboardBonesProjectionMode::YAxisViewVector,
            7 => BillboardBonesProjectionMode::YAxisViewPoint,
            _ => {
                return Err(Box::new(UnrecognizedFSKLBoneFlagProjectionMode {
                    value: (raw_bits & 0b00000000_00000111_00000000_00000000) >> 16,
                }))
            }
        };
        let transformation_flags = TransformationFlags::indirect_import(
            (raw_bits & 0b00001111_10000000_00000000_00000000) >> 23,
        );
        let bone_hierarchy_flags = BoneHierarchyFlags::indirect_import(
            (raw_bits & 0b11110000_00000000_00000000_00000000) >> 28,
        );
        Ok(Flags {
            visible,
            rotation,
            projection_mode,
            transformation_flags,
            bone_hierarchy_flags,
        })
    }
}

impl TransformationFlags {
    fn indirect_import(raw_bits: u32) -> TransformationFlags {
        let segment_scale_compensation = match raw_bits & 0b00001 {
            1 => true,
            _ => false,
        };
        let scale_uniformly = match raw_bits & 0b00010 {
            2 => true,
            _ => false,
        };
        let scale_volume_by_1 = match raw_bits & 0b00100 {
            4 => true,
            _ => false,
        };
        let no_rotation = match raw_bits & 0b01000 {
            8 => true,
            _ => false,
        };
        let no_translation = match raw_bits & 0b10000 {
            16 => true,
            _ => false,
        };
        TransformationFlags {
            segment_scale_compensation,
            scale_uniformly,
            scale_volume_by_1,
            no_rotation,
            no_translation,
        }
    }
}

impl BoneHierarchyFlags {
    fn indirect_import(raw_bits: u32) -> BoneHierarchyFlags {
        let scale_uniformly = match raw_bits & 0b0001 {
            1 => true,
            _ => false,
        };
        let scale_volume_by_1 = match raw_bits & 0b0010 {
            2 => true,
            _ => false,
        };
        let no_rotation = match raw_bits & 0b0100 {
            4 => true,
            _ => false,
        };
        let no_translation = match raw_bits & 0b1000 {
            8 => true,
            _ => false,
        };
        BoneHierarchyFlags {
            scale_uniformly,
            scale_volume_by_1,
            no_rotation,
            no_translation,
        }
    }
}
