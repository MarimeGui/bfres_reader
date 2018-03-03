extern crate bfres;
extern crate yaz0lib_rust;

use bfres::fres::FRESFile;
use bfres::fres::SubFileType;
use std::env;
use std::io::BufReader;
use std::io::Cursor;
use std::fs::File;
use std::path::Path;

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
        let output = yaz0lib_rust::decompress(&mut input_file_buf_reader);
        let mut bfres_cursor: Cursor<Vec<u8>> = Cursor::new(output);
        let bfres_file = FRESFile::read(&mut bfres_cursor).unwrap();
        println!("Read File successfully !");
        let info = bfres_file.get_sub_file_info().unwrap();
        for sub_file_info in info {
            println!("--- {}", sub_file_info.name);
            match sub_file_info.file_type {
                SubFileType::ModelData => println!("    Model data"),
                SubFileType::TextureData => println!("    Texture data"),
                _ => println!("    Other")
            }
            println!("    Position: {}", sub_file_info.position);
        }
    }
}