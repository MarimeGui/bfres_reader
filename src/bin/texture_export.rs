extern crate bcndecode;
extern crate bfres;
extern crate ez_io;
extern crate png;
extern crate yaz0lib_rust;

use bcndecode::{decode, BcnDecoderFormat, BcnEncoding};
use bfres::Importable;
use bfres::fres::FRES;
use bfres::ftex::FTEXFormat;
use bfres::swizzle::deswizzle;
use ez_io::ReadE;
use png::HasParameters;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor, Write};
use std::path::Path;

fn write_new_image(path: String, data: &[u8], width: u32, height: u32) {
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(data).unwrap();
}

fn write_raw_buffer(path: String, data: &[u8]) {
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    w.write_all(data).unwrap();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        let exec_name = args[0].to_string();
        println!(
            "Usage: ./{} input_file output_folder",
            Path::new(&exec_name).file_name().unwrap().to_str().unwrap()
        );
    } else if args.len() > 3 {
        println!("Please only give two arguments");
    } else {
        // Input the data
        let input_file = args[1].to_string();
        let output_folder = args[2].to_string();
        let mut input_file_reader =
            BufReader::new(File::open(&input_file).expect("Failed to open file for reading"));

        // Decompress
        println!("Decompressing...");
        let output = yaz0lib_rust::decompress(&mut input_file_reader).unwrap();
        println!("Decompressed !");

        // Make the cursor
        let mut bfres_cursor: Cursor<Vec<u8>> = Cursor::new(output);
        let bfres_cursor_ref = &mut bfres_cursor;

        // Import the file
        let bfres_file = FRES::import(bfres_cursor_ref).expect("Failed to read FRES file properly");

        // If some FTEX Sub-File exists
        if let Some(ftex_index_group) = bfres_file.sub_file_index_groups.texture_data {
            // Go through all of them
            for ftex_entry in ftex_index_group.entries {
                // Get the name
                let ftex_name = ftex_entry
                    .get_name(bfres_cursor_ref)
                    .expect("Failed to read FTEX name");

                // Load the FTEX
                let ftex = ftex_entry
                    .get_data(bfres_cursor_ref)
                    .expect("Failed to read FTEX");

                println!("\n{}, Format {}", ftex_name, ftex.header.texture_format);

                // Check for depth
                if ftex.header.texture_depth != 1 {
                    println!("Depth is not 1, skipping...");
                    continue;
                }

                // Get important info
                let width = ftex.header.texture_width as usize;
                let height = ftex.header.texture_height as usize;
                let encoding = match ftex.header.texture_format {
                    FTEXFormat::TcsR8G8B8A8Unorm => None,
                    FTEXFormat::TBc1Unorm | FTEXFormat::TBc1Srgb => Some(BcnEncoding::Bc1),
                    FTEXFormat::TBc2Unorm | FTEXFormat::TBc2Srgb => Some(BcnEncoding::Bc2),
                    FTEXFormat::TBc3Unorm | FTEXFormat::TBc3Srgb => Some(BcnEncoding::Bc3),
                    FTEXFormat::TBc4Unorm | FTEXFormat::TBc4Snorm => Some(BcnEncoding::Bc4),
                    FTEXFormat::TBc5Unorm | FTEXFormat::TBc5Snorm => Some(BcnEncoding::Bc5),
                    _ => {
                        println!("{} not implemented", ftex.header.texture_format);
                        continue;
                    }
                };

                // Read the whole buffer
                let mut raw_data = Vec::with_capacity(ftex.header.data_length as usize);
                ftex.header
                    .data_offset
                    .seek_abs_pos(bfres_cursor_ref)
                    .unwrap();
                while raw_data.len() < raw_data.capacity() {
                    raw_data.push(bfres_cursor_ref.read_to_u8().unwrap());
                }

                let de_swizzled_data = deswizzle(&ftex, &raw_data).expect("Failed to de swizzle");

                // Decode the buffer or not
                let image_data = match encoding {
                    Some(enc) => decode(
                        &de_swizzled_data,
                        width,
                        height,
                        enc,
                        BcnDecoderFormat::RGBA,
                    ).unwrap(),
                    None => de_swizzled_data,
                };

                let expected_output_len = width * height * 4;
                let real_output_len = image_data.len();

                // Write that to a file
                let output_base = format!("{}/{}", output_folder, ftex_name);
                if expected_output_len != real_output_len {
                    println!(
                        "  /!\\ Unexpected output size, writing the raw buffer ({} != {})",
                        expected_output_len, real_output_len
                    );
                    println!("  ->  Output as {}.raw", output_base);
                    write_raw_buffer(format!("{}.raw", output_base), &image_data);
                } else {
                    println!("  ->  Output as {}.png", output_base);
                    write_new_image(
                        format!("{}.png", output_base),
                        &image_data,
                        width as u32,
                        height as u32,
                    );
                }
            }
        } else {
            println!("No FTEX in this BFRES!");
        }
    }
}
