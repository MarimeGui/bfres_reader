use ez_io::ReadE;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::collections::HashMap;
use error::WrongMagicNumber;
use error::NoEntryForKey;
use util::RelativePointer;
use util::align_on_4_bytes;

pub struct FRESFile {
    pub header: FRESHeader,
    pub string_map: HashMap<u64, String>,
    pub sub_file_index_groups: Vec<SubFileIndexGroup>
}

pub struct FRESHeader {
    pub magic_number: [u8; 4],
    pub version: u32,
    pub bom: u16,
    pub header_length: u16,
    pub file_length: u32,
    pub file_alignment: u32,
    pub file_name_offset: RelativePointer,
    pub string_table_length: i32,
    pub string_table_offset: RelativePointer,
    pub sub_file_index_groups_offsets: [RelativePointer; 12],
    pub sub_file_index_groups_entry_counts: [u16; 12],
    pub user_pointer: u32
}

pub struct SubFileIndexGroup {
    pub file_type: SubFileType,
    pub entries: Vec<SubFileIndexGroupEntry>
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

pub struct SubFileIndexGroupEntry {
    pub search_value: u32,
    pub left_index: u16,
    pub right_index: u16,
    pub name_pointer: RelativePointer,
    pub data_pointer: RelativePointer
}

pub struct SubFileInfo {
    pub name: String,
    pub file_type: SubFileType,
    pub position: u64
}

impl FRESFile {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<FRESFile, Box<Error>> {
        let header = FRESHeader::read(reader)?;
        let string_map = read_string_map(&header, reader)?;
        let sub_file_index_groups = read_sub_file_index_groups(&header, reader)?;
        Ok(FRESFile {
            header,
            string_map,
            sub_file_index_groups
        })
    }
    pub fn get_sub_file_info(&self) -> Result<Vec<SubFileInfo>, Box<Error>> {
        let mut sub_file_info: Vec<SubFileInfo> = Vec::new();
        for sub_file_index_group in &self.sub_file_index_groups {
            let file_type = &sub_file_index_group.file_type;
            for entry in &sub_file_index_group.entries {
                let name_pos = entry.name_pointer.absolute_position()?;
                let name = match self.string_map.get(&name_pos) {
                    None => return Err(Box::new(NoEntryForKey{})),
                    Some(name) => name
                };
                let data_pos = entry.data_pointer.absolute_position()?;
                sub_file_info.push(SubFileInfo {
                    name: name.clone(),
                    file_type: file_type.clone(),
                    position: data_pos
                })
            }
        }
        Ok(sub_file_info)
    }
}

impl FRESHeader {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<FRESHeader, Box<Error>> {
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
        let file_name_offset = RelativePointer {
            location: reader.seek(SeekFrom::Current(0))?,
            points_to: i64::from(reader.read_be_to_i32()?)
        };
        // String Table Length
        let string_table_length = reader.read_be_to_i32()?;
        // String Table Offset
        let string_table_offset = RelativePointer {
            location: reader.seek(SeekFrom::Current(0))?,
            points_to: i64::from(reader.read_be_to_i32()?)
        };
        // File Offsets
        let mut file_offsets: [RelativePointer; 12] = [ RelativePointer {
            location: 0,
            points_to: 0
        }; 12];
        for ptr in &mut file_offsets {
            ptr.location = reader.seek(SeekFrom::Current(0))?;
            ptr.points_to = i64::from(reader.read_be_to_i32()?);
        }
        // File Counts
        let mut file_counts = [0u16; 12];
        for data in &mut file_counts {
            *data = reader.read_be_to_u16()?;
        }
        // User Pointer
        let user_pointer = reader.read_be_to_u32()?;
        Ok(FRESHeader {
            magic_number,
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

fn read_string_map<R: Read + Seek>(header: &FRESHeader, reader: &mut R) -> Result<HashMap<u64, String>, Box<Error>> {
    let mut string_table: HashMap<u64, String> = HashMap::new();
    let string_table_absolute_pos = header.string_table_offset.absolute_position()?;
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
        string_table.insert(abs_text_pos, text);
    }
    Ok(string_table)
}

impl SubFileIndexGroup {
    pub fn read<R: Read + Seek>(nb_entries: u16, reader: &mut R) -> Result<Vec<SubFileIndexGroupEntry>, Box<Error>> {
        let mut sub_file_index_group_entries: Vec<SubFileIndexGroupEntry> = Vec::new();
        let end_of_group_absolute_pos = u64::from(reader.read_be_to_u32()?) + reader.seek(SeekFrom::Current(0))?;
        if i32::from(nb_entries) != reader.read_be_to_i32()? {
            panic!();
        }
        reader.seek(SeekFrom::Current(16))?;  // Skip root entry
        for _ in 0..nb_entries {
            sub_file_index_group_entries.push(SubFileIndexGroupEntry::read(reader)?);
        }
        if reader.seek(SeekFrom::Current(0))? > end_of_group_absolute_pos {
            panic!();
        }
        Ok(sub_file_index_group_entries)
    }
}

impl SubFileIndexGroupEntry {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<SubFileIndexGroupEntry, Box<Error>> {
        let search_value = reader.read_be_to_u32()?;
        let left_index = reader.read_be_to_u16()?;
        let right_index = reader.read_be_to_u16()?;
        let name_pointer = RelativePointer {
            location: reader.seek(SeekFrom::Current(0))?,
            points_to: i64::from(reader.read_be_to_i32()?)
        };
        let data_pointer = RelativePointer {
            location: reader.seek(SeekFrom::Current(0))?,
            points_to: i64::from(reader.read_be_to_i32()?)
        };
        Ok(SubFileIndexGroupEntry {
            search_value,
            left_index,
            right_index,
            name_pointer,
            data_pointer
        })
    }
}

fn read_sub_file_index_groups<R: Read + Seek>(header: &FRESHeader, reader: &mut R) -> Result<Vec<SubFileIndexGroup>, Box<Error>> {
    let mut sub_file_index_groups: Vec<SubFileIndexGroup> = Vec::new();
    for id in 0..12 {
        if header.sub_file_index_groups_offsets[id].points_to != 0 {
            reader.seek(SeekFrom::Start(header.sub_file_index_groups_offsets[id].absolute_position()?))?;
            sub_file_index_groups.push(
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
                    entries: SubFileIndexGroup::read(header.sub_file_index_groups_entry_counts[id], reader)?
                }
            );
        }
    }
    Ok(sub_file_index_groups)
}
