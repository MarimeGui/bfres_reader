extern crate bfres;
extern crate yaz0lib_rust;

use bfres::fres::FRESFile;
use bfres::fres::SubFileType;
use bfres::Importable;
use std::env;
use std::io::BufReader;
use std::io::Cursor;
use std::fs::File;
use std::path::Path;
use std::error::Error;

pub struct SubFileInfo {
    pub name: String,
    pub file_type: SubFileType,
    pub position: u64
}

fn get_sub_file_info(file: &FRESFile) -> Result<Vec<SubFileInfo>, Box<Error>> {
    let mut sub_file_info: Vec<SubFileInfo> = Vec::new();
    for sub_file_index_group in &file.sub_file_index_groups.groups {
        let file_type = &sub_file_index_group.file_type;
        for entry in &sub_file_index_group.group.entries {
            let name_pos = entry.name_pointer.get_abs_pos()?;
            let name = match file.string_table.map.get(&name_pos) {
                None => panic!("Missing key"),
                Some(name) => name
            };
            let data_pos = entry.data_pointer.get_abs_pos()?;
            sub_file_info.push(SubFileInfo {
                name: name.clone(),
                file_type: file_type.clone(),
                position: data_pos
            })
        }
    }
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
        let bfres_file = FRESFile::import(&mut bfres_cursor).unwrap();
        println!("Read File successfully !");
        let info = get_sub_file_info(&bfres_file).unwrap();
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