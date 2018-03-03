extern crate bfres;
extern crate yaz0lib_rust;

use bfres::fres::FRESFile;
use std::env;
use std::io::BufReader;
use std::io::Cursor;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        let exec_name = args[0].to_string();
        println!("Usage: ./{} input_file [output_file]", Path::new(&exec_name).file_name().unwrap().to_str().unwrap());
    } else if args.len() > 3 {
        println!("Please only give one or two arguments");
    } else {
        let input_file = args[1].to_string();
        let input_file_reader = File::open(&input_file).expect("Failed to open file for reading");
        let mut input_file_buf_reader = BufReader::new(input_file_reader);
        let output = yaz0lib_rust::decompress(&mut input_file_buf_reader);
        let mut bfres_cursor: Cursor<Vec<u8>> = Cursor::new(output);
        FRESFile::read(&mut bfres_cursor).unwrap();
        println!("Read File successfully !")
    }
}