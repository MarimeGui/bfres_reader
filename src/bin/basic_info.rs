extern crate bfres;
extern crate yaz0lib_rust;

use bfres::fres::FRES;
use bfres::Importable;
use std::env;
use std::io::{BufReader, Cursor};
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
        println!("Decompressing...");
        let output = yaz0lib_rust::decompress(&mut input_file_buf_reader);
        println!("Decompressed !");
        let mut bfres_cursor: Cursor<Vec<u8>> = Cursor::new(output);
        let bfres_file = FRES::import(&mut bfres_cursor).unwrap();
        println!("Read File successfully !");
        println!("{} sub-files", bfres_file.header.get_total_sub_file_count());
        match bfres_file.sub_file_index_groups.model_data {
            Some(a) => {
                println!("{} Model data sub-files", a.entries.len());
                for (count, fmdl_entry) in a.entries.iter().enumerate() {
                    println!("--- {} @ 0x{:x}", fmdl_entry.get_name(&mut bfres_cursor).unwrap(), fmdl_entry.data_pointer.get_abs_pos().unwrap());
                    let fmdl = fmdl_entry.get_data(&mut bfres_cursor).unwrap();
                    println!("    Total number of vertices: {}", fmdl.header.total_nb_vertices);
                    if count > 9 {
                        break
                    }
                }
            },
            None => {}
        }
        match bfres_file.sub_file_index_groups.texture_data {
            Some(a) => {
                println!("{} Texture data sub-files", a.entries.len());
                for (count, ftex_entry) in a.entries.iter().enumerate() {
                    println!("--- {} @ 0x{:x}", ftex_entry.get_name(&mut bfres_cursor).unwrap(), ftex_entry.data_pointer.get_abs_pos().unwrap());
                    let ftex = ftex_entry.get_data(&mut bfres_cursor).unwrap();
                    println!("    Resolution: {} x {}", ftex.header.texture_width, ftex.header.texture_height);
                    if count > 9 {
                        break
                    }
                }
            },
            None => {}
        }
    }
}