extern crate bfres;
extern crate yaz0lib_rust;

use bfres::IndexGroup;
use bfres::fres::FRES;
use bfres::Importable;
use std::env;
use std::io::{BufReader, Cursor, Read, Seek};
use std::fs::File;
use std::path::Path;
use std::error::Error;

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

pub struct SubFileInfo {
    pub name: String,
    pub file_type: SubFileType,
    pub position: u64
}

fn get_sub_file_info<R: Read + Seek>(reader: &mut R, file: &FRES) -> Result<Vec<SubFileInfo>, Box<Error>> {
    fn process_group<R: Read + Seek, I: Importable>(reader: &mut R, file_type: &SubFileType, group: &Option<IndexGroup<I>>, sub_file_info: &mut Vec<SubFileInfo>) -> Result<(), Box<Error>> {
        if let Some(ref a) = *group {
            for entry in &a.entries {
                sub_file_info.push(SubFileInfo {
                    name: entry.get_name(reader)?,
                    file_type: file_type.clone(),
                    position: entry.data_pointer.get_abs_pos()?
                })
            }
        }
        Ok(())
    }
    let mut sub_file_info: Vec<SubFileInfo> = Vec::with_capacity(file.header.get_total_sub_file_count() as usize);
    let mut file_type: SubFileType = SubFileType::ModelData;
    process_group(reader, &file_type, &file.sub_file_index_groups.model_data, &mut sub_file_info)?;
    file_type = SubFileType::TextureData;
    process_group(reader, &file_type, &file.sub_file_index_groups.texture_data, &mut sub_file_info)?;
    file_type = SubFileType::SkeletonAnimation;
    process_group(reader, &file_type, &file.sub_file_index_groups.skeleton_animation, &mut sub_file_info)?;
    file_type = SubFileType::ShaderParameters;
    process_group(reader, &file_type, &file.sub_file_index_groups.shader_parameters, &mut sub_file_info)?;
    file_type = SubFileType::ColorAnimation;
    process_group(reader, &file_type, &file.sub_file_index_groups.color_animation, &mut sub_file_info)?;
    file_type = SubFileType::TextureSRTAnimation;
    process_group(reader, &file_type, &file.sub_file_index_groups.texture_srt_animation, &mut sub_file_info)?;
    file_type = SubFileType::TexturePatternAnimation;
    process_group(reader, &file_type, &file.sub_file_index_groups.texture_pattern_animation, &mut sub_file_info)?;
    file_type = SubFileType::BoneVisibilityAnimation;
    process_group(reader, &file_type, &file.sub_file_index_groups.bone_visibility_animation, &mut sub_file_info)?;
    file_type = SubFileType::MaterialVisibilityAnimation;
    process_group(reader, &file_type, &file.sub_file_index_groups.material_visibility_animation, &mut sub_file_info)?;
    file_type = SubFileType::ShapeAnimation;
    process_group(reader, &file_type, &file.sub_file_index_groups.shape_animation, &mut sub_file_info)?;
    file_type = SubFileType::SceneAnimation;
    process_group(reader, &file_type, &file.sub_file_index_groups.scene_animation, &mut sub_file_info)?;
    file_type = SubFileType::Embedded;
    process_group(reader, &file_type, &file.sub_file_index_groups.embedded_file, &mut sub_file_info)?;
    Ok(sub_file_info)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        let exec_name = args[0].to_string();
        println!("Usage: ./{} input_file", Path::new(&exec_name).file_name().unwrap().to_str().unwrap());
    } else if args.len() > 2 {
        println!("Please only give one argument");
    } else {
        let input_file = args[1].to_string();
        let input_file_reader = File::open(&input_file).expect("Failed to open file for reading");
        let mut input_file_buf_reader = BufReader::new(input_file_reader);
        println!("Decompressing...");
        let output = yaz0lib_rust::decompress(&mut input_file_buf_reader);
        println!("Decompressed !");
        let mut bfres_cursor: Cursor<Vec<u8>> = Cursor::new(output);
        let bfres_file = FRES::import(&mut bfres_cursor).unwrap();
        println!("Read File successfully !");
        let info = get_sub_file_info(&mut bfres_cursor, &bfres_file).unwrap();
        println!("{} sub-files", &info.len());
        for (amount, sub_file_info) in info.iter().enumerate() {
            println!("--- {} @ 0x{:x}", sub_file_info.name, sub_file_info.position);
            match sub_file_info.file_type {
                SubFileType::ModelData => println!("    Model data"),
                SubFileType::TextureData => println!("    Texture data"),
                _ => println!("    Other")
            }
            if amount > 9 {
                break;
            }
        }
    }
}