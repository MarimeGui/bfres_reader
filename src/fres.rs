use ez_io::ReadE;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::collections::HashMap;
use IndexGroup;
use Importable;
use fmdl::FMDL;
use ftex::FTEX;
use fska::FSKA;
use fshu::FSHU;
use ftxp::FTXP;
use fvis::FVIS;
use fsha::FSHA;
use fscn::FSCN;
use embedded::Embedded;
use error::WrongMagicNumber;
use util::Pointer;
use util::align_on_4_bytes;

pub struct FRES {
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
    pub sub_file_index_groups_offsets: [Option<Pointer>; 12],
    pub sub_file_index_groups_entry_counts: [u16; 12],
    pub user_pointer: u32
}

pub struct StringTable {
    pub map: HashMap<u64, String>
}

pub struct SubFileIndexGroups {
    pub model_data: Option<IndexGroup<FMDL>>,
    pub texture_data: Option<IndexGroup<FTEX>>,
    pub skeleton_animation: Option<IndexGroup<FSKA>>,
    pub shader_parameters: Option<IndexGroup<FSHU>>,
    pub color_animation: Option<IndexGroup<FSHU>>,
    pub texture_srt_animation: Option<IndexGroup<FSHU>>,
    pub texture_pattern_animation: Option<IndexGroup<FTXP>>,
    pub bone_visibility_animation: Option<IndexGroup<FVIS>>,
    pub material_visibility_animation: Option<IndexGroup<FVIS>>,
    pub shape_animation: Option<IndexGroup<FSHA>>,
    pub scene_animation: Option<IndexGroup<FSCN>>,
    pub embedded_file: Option<IndexGroup<Embedded>>
}

impl Importable for FRES {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FRES, Box<Error>> {
        let header = FRESHeader::import(reader)?;
        let string_map = StringTable::import(&header, reader)?;
        let sub_file_index_groups = SubFileIndexGroups::import(&header, reader)?;
        Ok(FRES {
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
        let mut file_offsets: [Option<Pointer>; 12] = [ None; 12];
        for ptr in &mut file_offsets {
            let temp = Pointer::read_new_rel_i32_be(reader)?;
            if temp.points_to != 0 {
                *ptr = Some(temp);
            };
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

impl FRESHeader {
    pub fn get_total_sub_file_count(&self) -> u16 {
        let mut grand_total = 0u16;
        for count in &self.sub_file_index_groups_entry_counts {
            grand_total += count;
        }
        grand_total
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
        fn process_group<R: Read + Seek, I: Importable>(index_group_pointer: &Option<Pointer>, reader: &mut R) -> Result<Option<IndexGroup<I>>, Box<Error>> {
            Ok(match *index_group_pointer {
                Some(a) => {
                    a.seek_abs_pos(reader)?;
                    Some(IndexGroup::import(reader)?)
                },
                None => None
            })
        }
        let model_data: Option<IndexGroup<FMDL>> = process_group(&header.sub_file_index_groups_offsets[0], reader)?;
        let texture_data: Option<IndexGroup<FTEX>> = process_group(&header.sub_file_index_groups_offsets[1], reader)?;
        let skeleton_animation: Option<IndexGroup<FSKA>> = process_group(&header.sub_file_index_groups_offsets[2], reader)?;
        let shader_parameters: Option<IndexGroup<FSHU>> = process_group(&header.sub_file_index_groups_offsets[3], reader)?;
        let color_animation: Option<IndexGroup<FSHU>> = process_group(&header.sub_file_index_groups_offsets[4], reader)?;
        let texture_srt_animation: Option<IndexGroup<FSHU>> = process_group(&header.sub_file_index_groups_offsets[5], reader)?;
        let texture_pattern_animation: Option<IndexGroup<FTXP>> = process_group(&header.sub_file_index_groups_offsets[6], reader)?;
        let bone_visibility_animation: Option<IndexGroup<FVIS>> = process_group(&header.sub_file_index_groups_offsets[7], reader)?;
        let material_visibility_animation: Option<IndexGroup<FVIS>> = process_group(&header.sub_file_index_groups_offsets[8], reader)?;
        let shape_animation: Option<IndexGroup<FSHA>> = process_group(&header.sub_file_index_groups_offsets[9], reader)?;
        let scene_animation: Option<IndexGroup<FSCN>> = process_group(&header.sub_file_index_groups_offsets[10], reader)?;
        let embedded_file: Option<IndexGroup<Embedded>> = process_group(&header.sub_file_index_groups_offsets[11], reader)?;
        Ok(SubFileIndexGroups {
            model_data,
            texture_data,
            skeleton_animation,
            shader_parameters,
            color_animation,
            texture_srt_animation,
            texture_pattern_animation,
            bone_visibility_animation,
            material_visibility_animation,
            shape_animation,
            scene_animation,
            embedded_file
        })
    }
}