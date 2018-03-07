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
        println!("Version v{}.{}.{}.{}", bfres_file.header.version[0], bfres_file.header.version[1], bfres_file.header.version[2], bfres_file.header.version[3]);
        println!("{} sub-files", bfres_file.header.get_total_sub_file_count());
        // FMDL
        if let Some(a) = bfres_file.sub_file_index_groups.model_data {
            println!("{} FMDL sub-files", a.entries.len());
            for (count, fmdl_entry) in a.entries.iter().enumerate() {
                println!("--- {} @ 0x{:x}", fmdl_entry.get_name(&mut bfres_cursor).unwrap(), fmdl_entry.data_pointer.get_abs_pos().unwrap());
                let fmdl = fmdl_entry.get_data(&mut bfres_cursor).unwrap();
                println!("    {} vertices", fmdl.header.total_nb_vertices);
                // FVTX
                if !fmdl.fvtx_array.entries.is_empty() {
                    println!("    {} FVTX:", fmdl.fvtx_array.entries.len());
                    for fvtx_entry in fmdl.fvtx_array.entries {
                        println!("    --- @ 0x{:x}", fvtx_entry.data_pointer.get_abs_pos().unwrap());
                        let fvtx = fvtx_entry.get_data(&mut bfres_cursor).unwrap();
                        println!("        {} vertices", fvtx.header.nb_vertices);
                        println!("        {} attributes:", fvtx.header.attribute_count);
                        for attribute_entry in fvtx.attributes.entries {
                            println!("        --- {} @ 0x{:x}", attribute_entry.get_name(&mut bfres_cursor).unwrap(), attribute_entry.data_pointer.get_abs_pos().unwrap());
                            let attribute = attribute_entry.get_data(&mut bfres_cursor).unwrap();
                        }
                    }
                }
                // FMAT
                if !fmdl.fmat_index_group.entries.is_empty() {
                    println!("    {} FMAT:", fmdl.fmat_index_group.entries.len());
                    for fmat_entry in fmdl.fmat_index_group.entries {
                        println!("    --- {} @ 0x{:x}", fmat_entry.get_name(&mut bfres_cursor).unwrap(), fmat_entry.data_pointer.get_abs_pos().unwrap());
                        let fmat = fmat_entry.get_data(&mut bfres_cursor).unwrap();
                        println!("        {} texture references", fmat.header.texture_reference_count);
                    }
                }
                // FSKL
                println!("    1 FSKL:");
                println!("    --- @ 0x{:x}", fmdl.header.fskl_offset.get_abs_pos().unwrap());
                println!("        {} bones", fmdl.fskl.header.bone_array_count);
                // FSHP
                if !fmdl.fshp_index_group.entries.is_empty() {
                    println!("    {} FSHP:", fmdl.fshp_index_group.entries.len());
                    for fshp_entry in fmdl.fshp_index_group.entries {
                        println!("    --- {} @ 0x{:x}", fshp_entry.get_name(&mut bfres_cursor).unwrap(), fshp_entry.data_pointer.get_abs_pos().unwrap());
                        let fshp = fshp_entry.get_data(&mut bfres_cursor).unwrap();
                        println!("        {} skin vertices", fshp.header.vertex_skin_count);
                    }
                }
                if count > 9 {
                    break
                }
            }
        }
        if let Some(a) = bfres_file.sub_file_index_groups.texture_data {
            println!("{} FTEX sub-files", a.entries.len());
            for (count, ftex_entry) in a.entries.iter().enumerate() {
                println!("--- {} @ 0x{:x}", ftex_entry.get_name(&mut bfres_cursor).unwrap(), ftex_entry.data_pointer.get_abs_pos().unwrap());
                let ftex = ftex_entry.get_data(&mut bfres_cursor).unwrap();
                println!("    Resolution: {} x {}", ftex.header.texture_width, ftex.header.texture_height);
                if count > 9 {
                    break
                }
            }
        }
        if let Some(a) = bfres_file.sub_file_index_groups.embedded_file {
            println!("{} Embedded sub-files", a.entries.len());
            for (count, embedded_entry) in a.entries.iter().enumerate() {
                println!("--- {} @ 0x{:x}", embedded_entry.get_name(&mut bfres_cursor).unwrap(), embedded_entry.data_pointer.get_abs_pos().unwrap());
                let embedded = embedded_entry.get_data(&mut bfres_cursor).unwrap();
                println!("    File @ 0x{:x}", embedded.offset.get_abs_pos().unwrap());
                println!("    {} byte{} long", embedded.length, if embedded.length == 1 {""} else {"s"});
                if count > 9 {
                    break
                }
            }
        }
    }
}