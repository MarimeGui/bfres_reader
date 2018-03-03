use ez_io::ReadE;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::collections::HashMap;
use error::WrongMagicNumber;
use error::NoEntryForKey;

pub fn get_sub_file_info(file: FRESFile) -> Result<Vec<SubFileInfo>, Box<Error>> {
    let mut sub_file_info: Vec<SubFileInfo> = Vec::new();
    let mut s_f_i_g_index = 0;
    for s_f_i_g_offset in file.header.sub_file_index_groups_offsets.iter() {
        if *s_f_i_g_offset != 0 {
            for entry in match file.sub_file_index_groups[s_f_i_g_index] {
                SubFileIndexGroup::ModelData(ref a) => a,
                SubFileIndexGroup::TextureData(ref a) => a,
                SubFileIndexGroup::SkeletonAnimation(ref a) => a,
                SubFileIndexGroup::ShaderParameters(ref a) => a,
                SubFileIndexGroup::ColorAnimation(ref a) => a,
                SubFileIndexGroup::TextureSRTAnimation(ref a) => a,
                SubFileIndexGroup::TexturePatternAnimation(ref a) => a,
                SubFileIndexGroup::BoneVisibilityAnimation(ref a) => a,
                SubFileIndexGroup::MaterialVisibilityAnimation(ref a) => a,
                SubFileIndexGroup::ShapeAnimation(ref a) => a,
                SubFileIndexGroup::SceneAnimation(ref a) => a,
                SubFileIndexGroup::Embedded(ref a) => a
            } {
                let name_pos = entry.name_pointer as u64 + entry.meta_absolute_pos + 8;
                let data_pos = entry.data_pointer as u64 + entry.meta_absolute_pos + 12;
                match file.string_map.get(&name_pos) {
                    None => return Err(Box::new(NoEntryForKey{})),
                    Some(name) => {sub_file_info.push(SubFileInfo {
                        name: name.clone(),
                        position: data_pos as u64
                    })}
                }
                ;
            };
            s_f_i_g_index += 1;
        }
    }
    Ok(sub_file_info)
}

pub struct FRESFile {
    pub header: FRESHeader,
    pub string_map: HashMap<u64, String>,
    pub sub_file_index_groups: Vec<SubFileIndexGroup>
}

impl FRESFile {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<FRESFile, Box<Error>> {
        let header = FRESHeader::read(reader)?;
        let string_map = read_string_table(&header, reader)?;
        let sub_file_index_groups = read_sub_file_index_groups(&header, reader)?;
        Ok(FRESFile {
            header,
            string_map,
            sub_file_index_groups
        })
    }
}

pub struct SubFileInfo {
    pub name: String,
    pub position: u64
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
    pub sub_file_index_groups_offsets: [i32; 12],
    pub sub_file_index_groups_entry_counts: [u16; 12],
    pub user_pointer: u32
}

impl FRESHeader {
    pub fn read<R: Read>(reader: &mut R) -> Result<FRESHeader, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        if magic_number != [b'F', b'R', b'E', b'S'] {
            return Err(Box::new(WrongMagicNumber{}))
        }
        let version = reader.read_be_to_u32()?;
        let bom = reader.read_be_to_u16()?;
        if bom != 0xFEFF {
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
        for id in 0..12 {
            file_offsets[id] = reader.read_be_to_i32()?;
        }
        let mut file_counts = [0u16; 12];
        for id in 0..12 {
            file_counts[id] = reader.read_be_to_u16()?;
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
            sub_file_index_groups_offsets: file_offsets,
            sub_file_index_groups_entry_counts: file_counts,
            user_pointer
        })
    }
}

fn read_string_table<R: Read + Seek>(header: &FRESHeader, reader: &mut R) -> Result<HashMap<u64, String>, Box<Error>> {
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

fn align_on_4_bytes<R: Read + Seek>(reader: &mut R) -> Result<(), Box<Error>> {
    let pos = reader.seek(SeekFrom::Current(0))?;
    if pos % 4 != 0 {
        reader.seek(SeekFrom::Current((4 - (pos % 4)) as i64))?;
    }
    Ok(())
}

fn read_sub_file_index_groups<R: Read + Seek>(header: &FRESHeader, reader: &mut R) -> Result<Vec<SubFileIndexGroup>, Box<Error>> {
    let mut sub_file_index_groups: Vec<SubFileIndexGroup> = Vec::new();
    let mut index = 0;
    for offset in header.sub_file_index_groups_offsets.iter() {
        if *offset != 0 {
            reader.seek(SeekFrom::Start(0x20 + ((index*4) as u64) + (*offset as u64)))?;
            sub_file_index_groups.push(match index {
                0 => SubFileIndexGroup::ModelData(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                1 => SubFileIndexGroup::TextureData(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                2 => SubFileIndexGroup::SkeletonAnimation(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                3 => SubFileIndexGroup::ShaderParameters(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                4 => SubFileIndexGroup::ColorAnimation(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                5 => SubFileIndexGroup::TextureSRTAnimation(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                6 => SubFileIndexGroup::TexturePatternAnimation(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                7 => SubFileIndexGroup::BoneVisibilityAnimation(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                8 => SubFileIndexGroup::MaterialVisibilityAnimation(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                9 => SubFileIndexGroup::ShapeAnimation(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                10 => SubFileIndexGroup::SceneAnimation(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                11 => SubFileIndexGroup::Embedded(read_sub_file_index_group(header.sub_file_index_groups_entry_counts[index], reader)?),
                _ => panic!()
            });
        }
        index += 1;
    }
    Ok(sub_file_index_groups)
}

fn read_sub_file_index_group<R: Read + Seek>(nb_entries: u16, reader: &mut R) -> Result<Vec<SubFileIndexGroupEntry>, Box<Error>> {
    let mut sub_file_index_group_entries: Vec<SubFileIndexGroupEntry> = Vec::new();
    let end_of_group_absolute_pos = reader.read_be_to_u32()? as u64 + reader.seek(SeekFrom::Current(0))?;
    if nb_entries as i32 != reader.read_be_to_i32()? {
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

pub struct SubFileIndexGroupEntry {
    pub meta_absolute_pos: u64,  // Makes my life about ten times easier
    pub search_value: u32,
    pub left_index: u16,
    pub right_index: u16,
    pub name_pointer: i32,
    pub data_pointer: i32
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

pub enum SubFileIndexGroup {
    ModelData(Vec<SubFileIndexGroupEntry>),
    TextureData(Vec<SubFileIndexGroupEntry>),
    SkeletonAnimation(Vec<SubFileIndexGroupEntry>),
    ShaderParameters(Vec<SubFileIndexGroupEntry>),
    ColorAnimation(Vec<SubFileIndexGroupEntry>),
    TextureSRTAnimation(Vec<SubFileIndexGroupEntry>),
    TexturePatternAnimation(Vec<SubFileIndexGroupEntry>),
    BoneVisibilityAnimation(Vec<SubFileIndexGroupEntry>),
    MaterialVisibilityAnimation(Vec<SubFileIndexGroupEntry>),
    ShapeAnimation(Vec<SubFileIndexGroupEntry>),
    SceneAnimation(Vec<SubFileIndexGroupEntry>),
    Embedded(Vec<SubFileIndexGroupEntry>)
}
