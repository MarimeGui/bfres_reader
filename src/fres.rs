use ez_io::ReadE;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::collections::HashMap;
use error::WrongMagicNumber;
use error::NoEntryForKey;

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
    pub file_name_offset: i32,
    pub string_table_length: i32,
    pub string_table_offset: i32,
    pub meta_sub_file_index_groups_offsets_positions: [u64; 12],
    pub sub_file_index_groups_offsets: [i32; 12],
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
    pub meta_absolute_pos: u64,  // Makes my life about ten times easier
    pub search_value: u32,
    pub left_index: u16,
    pub right_index: u16,
    pub name_pointer: i32,
    pub data_pointer: i32
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
                let name_pos = entry.meta_absolute_pos + 8 + entry.name_pointer as u64;
                let name = match self.string_map.get(&name_pos) {
                    None => return Err(Box::new(NoEntryForKey{})),
                    Some(name) => name
                };
                let data_pos = entry.meta_absolute_pos + 12 + entry.data_pointer as u64;
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
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        if magic_number != [b'F', b'R', b'E', b'S'] {
            return Err(Box::new(WrongMagicNumber{}))
        }
        let version = reader.read_be_to_u32()?;
        let bom = reader.read_be_to_u16()?;
        if bom != 0xFEFF {
            // Not supposed to see little-endian on the console, returning error here for convenience
            return Err(Box::new(WrongMagicNumber{}))
        }
        let header_length = reader.read_be_to_u16()?;
        if header_length != 0x0010 {
            return Err(Box::new(WrongMagicNumber{}))
        }
        let file_length = reader.read_be_to_u32()?;
        let file_alignment = reader.read_be_to_u32()?;
        let file_name_offset = reader.read_be_to_i32()?;
        let string_table_length = reader.read_be_to_i32()?;
        let string_table_offset = reader.read_be_to_i32()?;
        let mut file_offsets = [0i32; 12];
        let mut meta_sub_file_index_groups_offsets_positions = [0u64; 12];
        for id in 0..12 {
            meta_sub_file_index_groups_offsets_positions[id] = reader.seek(SeekFrom::Current(0))?;
            file_offsets[id] = reader.read_be_to_i32()?;
        }
        let mut file_counts = [0u16; 12];
        for data in &mut file_counts {
            *data = reader.read_be_to_u16()?;
        }
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
            meta_sub_file_index_groups_offsets_positions,
            sub_file_index_groups_offsets: file_offsets,
            sub_file_index_groups_entry_counts: file_counts,
            user_pointer
        })
    }
}

fn read_string_map<R: Read + Seek>(header: &FRESHeader, reader: &mut R) -> Result<HashMap<u64, String>, Box<Error>> {
    let mut string_table: HashMap<u64, String> = HashMap::new();
    let string_table_absolute_pos = 0x1Cu64 + header.string_table_offset as u64;
    let string_table_end_absolute_pos = string_table_absolute_pos + header.string_table_length as u64;
    reader.seek(SeekFrom::Start(string_table_absolute_pos))?;
    while reader.seek(SeekFrom::Current(0))? < string_table_end_absolute_pos {
        align_on_4_bytes(reader)?;
        let length = reader.read_be_to_u32()?;
        let abs_text_pos = reader.seek(SeekFrom::Current(0))?;
        if length == 0 {
            break
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
        let meta_absolute_pos = reader.seek(SeekFrom::Current(0))?;
        let search_value = reader.read_be_to_u32()?;
        let left_index = reader.read_be_to_u16()?;
        let right_index = reader.read_be_to_u16()?;
        let name_pointer = reader.read_be_to_i32()?;
        let data_pointer = reader.read_be_to_i32()?;
        Ok(SubFileIndexGroupEntry {
            meta_absolute_pos,
            search_value,
            left_index,
            right_index,
            name_pointer,
            data_pointer
        })
    }
}

fn align_on_4_bytes<R: Read + Seek>(reader: &mut R) -> Result<(), Box<Error>> {
    let pos = reader.seek(SeekFrom::Current(0))?;
    if pos % 4 != 0 {
        reader.seek(SeekFrom::Current((4 - (pos % 4)) as i64))?;
    }
    Ok(())
}

fn read_sub_file_index_groups<R: Read + Seek>(header: &FRESHeader, reader: &mut R) -> Result<Vec<SubFileIndexGroup>, Box<Error>> {
    let mut sub_file_index_groups: Vec<SubFileIndexGroup> = Vec::new();
    for id in 0..12 {
        if header.sub_file_index_groups_offsets[id] != 0 {
            let actual_offset = header.sub_file_index_groups_offsets[id] as u64 + header.meta_sub_file_index_groups_offsets_positions[id];
            reader.seek(SeekFrom::Start(actual_offset))?;
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
