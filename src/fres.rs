use ez_io::ReadE;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::collections::HashMap;
use IndexGroup;
use Importable;
use error::WrongMagicNumber;
use util::Pointer;
use util::align_on_4_bytes;

pub struct FRESFile {
    pub header: FRESHeader,
    pub string_table: StringTable,
    pub sub_file_index_groups: SubFileIndexGroups
}

pub struct FRESHeader {
    pub version: u32,
    pub bom: u16,
    pub header_length: u16,
    pub file_length: u32,
    pub file_alignment: u32,
    pub file_name_offset: Pointer,
    pub string_table_length: i32,
    pub string_table_offset: Pointer,
    pub sub_file_index_groups_offsets: [Pointer; 12],
    pub sub_file_index_groups_entry_counts: [u16; 12],
    pub user_pointer: u32
}

pub struct StringTable {
    pub map: HashMap<u64, String>
}

pub struct SubFileIndexGroups {
    pub groups: Vec<SubFileIndexGroup>
}

pub struct SubFileIndexGroup {
    pub file_type: SubFileType,
    pub group: IndexGroup
}

#[derive(Clone)]
pub enum SubFileType {
    ModelData,
    TextureData,
    SkeletonAnimation,
    ShaderParameters,
    ColorAnimation,
    TextureSRTAnimation,
    TexturePatternAnimation,
    BoneVisibilityAnimation,
    MaterialVisibilityAnimation,
    ShapeAnimation,
    SceneAnimation,
    Embedded
}

impl Importable for FRESFile {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FRESFile, Box<Error>> {
        let header = FRESHeader::import(reader)?;
        let string_map = StringTable::import(&header, reader)?;
        let sub_file_index_groups = SubFileIndexGroups::import(&header, reader)?;
        Ok(FRESFile {
            header,
            string_table: string_map,
            sub_file_index_groups
        })
    }
}

impl Importable for FRESHeader {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FRESHeader, Box<Error>> {
        // Magic Number
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        if magic_number != [b'F', b'R', b'E', b'S'] {
            return Err(Box::new(WrongMagicNumber{}))
        }
        // Version
        let version = reader.read_be_to_u32()?;
        // Byte Order Mark
        let bom = reader.read_be_to_u16()?;
        if bom != 0xFEFF {
            // Not supposed to see little-endian on the console, returning error here for convenience
            return Err(Box::new(WrongMagicNumber{}))
        }
        // Header Length
        let header_length = reader.read_be_to_u16()?;
        if header_length != 0x0010 {
            // Again, not supposed to be bigger than 16 bytes
            return Err(Box::new(WrongMagicNumber{}))
        }
        // File Length
        let file_length = reader.read_be_to_u32()?;
        // File Alignment
        let file_alignment = reader.read_be_to_u32()?;
        // File Name Offset
        let file_name_offset = Pointer::read_new_rel_i32_be(reader)?;
        // String Table Length
        let string_table_length = reader.read_be_to_i32()?;
        // String Table Offset
        let string_table_offset = Pointer::read_new_rel_i32_be(reader)?;
        // File Offsets
        let mut file_offsets: [Pointer; 12] = [ Pointer {
            location: None,
            points_to: 0
        }; 12];
        for ptr in &mut file_offsets {
            *ptr = Pointer::read_new_rel_i32_be(reader)?;
        }
        // File Counts
        let mut file_counts = [0u16; 12];
        for data in &mut file_counts {
            *data = reader.read_be_to_u16()?;
        }
        // User Pointer
        let user_pointer = reader.read_be_to_u32()?;
        Ok(FRESHeader {
            version,
            bom,
            header_length,
            file_length,
            file_alignment,
            file_name_offset,
            string_table_length,
            string_table_offset,
            sub_file_index_groups_offsets: file_offsets,
            sub_file_index_groups_entry_counts: file_counts,
            user_pointer
        })
    }
}

impl StringTable {
    fn import<R: Read + Seek>(header: &FRESHeader, reader: &mut R) -> Result<StringTable, Box<Error>> {
        let mut map: HashMap<u64, String> = HashMap::new();
        let string_table_absolute_pos = header.string_table_offset.get_abs_pos()?;
        let string_table_end_absolute_pos = string_table_absolute_pos + header.string_table_length as u64;
        reader.seek(SeekFrom::Start(string_table_absolute_pos))?;
        while reader.seek(SeekFrom::Current(0))? < string_table_end_absolute_pos {
            align_on_4_bytes(reader)?;
            let length = reader.read_be_to_u32()?;
            let abs_text_pos = reader.seek(SeekFrom::Current(0))?;
            if length == 0 {
                continue
            }
            let text = reader.read_to_string_n(length)?;
            map.insert(abs_text_pos, text);
        }
        Ok(StringTable {
            map
        })
    }
}

impl SubFileIndexGroups {
    fn import<R: Read + Seek>(header: &FRESHeader, reader: &mut R) -> Result<SubFileIndexGroups, Box<Error>> {
        let mut groups: Vec<SubFileIndexGroup> = Vec::with_capacity(12);
        for id in 0..12 {
            if header.sub_file_index_groups_offsets[id].points_to != 0 {
                header.sub_file_index_groups_offsets[id].seek_abs_pos(reader)?;
                groups.push(
                    SubFileIndexGroup {
                        file_type: match id {
                            0 => SubFileType::ModelData,
                            1 => SubFileType::TextureData,
                            2 => SubFileType::SkeletonAnimation,
                            3 => SubFileType::ShaderParameters,
                            4 => SubFileType::ColorAnimation,
                            5 => SubFileType::TextureSRTAnimation,
                            6 => SubFileType::TexturePatternAnimation,
                            7 => SubFileType::BoneVisibilityAnimation,
                            8 => SubFileType::MaterialVisibilityAnimation,
                            9 => SubFileType::ShapeAnimation,
                            10 => SubFileType::SceneAnimation,
                            11 => SubFileType::Embedded,
                            _ => panic!()
                        },
                        group: IndexGroup::import(reader)?
                    }
                );
            }
        }
        Ok(SubFileIndexGroups {
            groups
        })
    }
}