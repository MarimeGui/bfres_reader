extern crate bfres;
extern crate yaz0lib_rust;

use bfres::Importable;
use bfres::fres::FRES;
use std::env;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        let exec_name = args[0].to_string();
        println!(
            "Usage: ./{} input_file",
            Path::new(&exec_name).file_name().unwrap().to_str().unwrap()
        );
    } else if args.len() > 2 {
        println!("Please only give one argument");
    } else {
        let input_file = args[1].to_string();
        let input_file_reader = File::open(&input_file).expect("Failed to open file for reading");
        let mut input_file_buf_reader = BufReader::new(input_file_reader);
        let mut yaz_check_buffer = [0u8; 4];
        input_file_buf_reader
            .read_exact(&mut yaz_check_buffer)
            .expect("Failed to read first Magic Number");
        input_file_buf_reader
            .seek(SeekFrom::Start(0))
            .expect("Failed to re-seek to beginning of the file");
        let mut bfres_cursor = if yaz_check_buffer == [b'Y', b'a', b'z', b'0'] {
            Cursor::new(
                yaz0lib_rust::decompress(&mut input_file_buf_reader).expect("Failed to decompress"),
            )
        } else {
            let mut bfres_data = Vec::new();
            input_file_buf_reader
                .read_to_end(&mut bfres_data)
                .expect("Failed to read all data");
            Cursor::new(bfres_data)
        };
        let bfres_file = FRES::import(&mut bfres_cursor).unwrap();
        println!("Read File successfully !");
        println!("Version {}", bfres_file.header.version);
        println!("{} sub-files", bfres_file.header.get_total_sub_file_count());
        // FMDL
        if let Some(a) = bfres_file.sub_file_index_groups.model_data {
            println!("{} FMDL sub-files", a.entries.len());
            for fmdl_entry in a.entries {
                println!(
                    "--- {} @ 0x{:x}",
                    fmdl_entry.get_name(&mut bfres_cursor).unwrap(),
                    fmdl_entry.data_pointer.get_abs_pos().unwrap()
                );
                let fmdl = fmdl_entry.get_data(&mut bfres_cursor).unwrap();
                println!("    {} vertices", fmdl.header.total_nb_vertices);
                // FVTX
                if !fmdl.fvtx_array.entries.is_empty() {
                    println!("    {} FVTX:", fmdl.fvtx_array.entries.len());
                    for fvtx_entry in fmdl.fvtx_array.entries {
                        println!(
                            "    --- @ 0x{:x}",
                            fvtx_entry.data_pointer.get_abs_pos().unwrap()
                        );
                        let fvtx = fvtx_entry.get_data(&mut bfres_cursor).unwrap();
                        println!("        {} vertices", fvtx.header.nb_vertices);
                        println!("        {} attributes:", fvtx.header.attribute_count);
                        for attribute_entry in fvtx.attributes_index_group.entries {
                            println!(
                                "        --- {} @ 0x{:x}",
                                attribute_entry.get_name(&mut bfres_cursor).unwrap(),
                                attribute_entry.data_pointer.get_abs_pos().unwrap()
                            );
                            let attribute = attribute_entry.get_data(&mut bfres_cursor).unwrap();
                            println!("            Format: {}", attribute.format);
                            println!(
                                "            Buffer Info ID: {}",
                                attribute.buffer_info_index
                            );
                            println!("            Buffer Offset: {}", attribute.buffer_offset);
                        }
                        println!("        {} buffer info:", fvtx.header.buffer_info_count);
                        for buffer_info_entry in fvtx.buffer_info_array.entries {
                            println!(
                                "        --- @ 0x{:x}",
                                buffer_info_entry.data_pointer.get_abs_pos().unwrap()
                            );
                            let buffer_info =
                                buffer_info_entry.get_data(&mut bfres_cursor).unwrap();
                            println!("            {} bytes long", buffer_info.size);
                            println!("            Stride: {}", buffer_info.stride);
                        }
                    }
                }
                // FMAT
                if !fmdl.fmat_index_group.entries.is_empty() {
                    println!("    {} FMAT:", fmdl.fmat_index_group.entries.len());
                    for fmat_entry in fmdl.fmat_index_group.entries {
                        println!(
                            "    --- {} @ 0x{:x}",
                            fmat_entry.get_name(&mut bfres_cursor).unwrap(),
                            fmat_entry.data_pointer.get_abs_pos().unwrap()
                        );
                        let fmat = fmat_entry.get_data(&mut bfres_cursor).unwrap();
                        println!(
                            "        {} texture references",
                            fmat.header.texture_reference_count
                        );
                    }
                }
                // FSKL
                println!("    1 FSKL:");
                println!(
                    "    --- @ 0x{:x}",
                    fmdl.header.fskl_offset.get_abs_pos().unwrap()
                );
                println!("        {} bones", fmdl.fskl.header.bone_array_count);
                // FSHP
                if !fmdl.fshp_index_group.entries.is_empty() {
                    println!("    {} FSHP:", fmdl.fshp_index_group.entries.len());
                    for fshp_entry in fmdl.fshp_index_group.entries {
                        println!(
                            "    --- {} @ 0x{:x}",
                            fshp_entry.get_name(&mut bfres_cursor).unwrap(),
                            fshp_entry.data_pointer.get_abs_pos().unwrap()
                        );
                        let fshp = fshp_entry.get_data(&mut bfres_cursor).unwrap();
                        println!("        Flags: {}", fshp.header.flags);
                        println!("        {} LOD Models:", fshp.header.lod_model_count);
                        for lod_entry in fshp.lod_model_array.entries {
                            println!(
                                "        --- @ 0x{:x}",
                                lod_entry.data_pointer.get_abs_pos().unwrap()
                            );
                            let lod = lod_entry.get_data(&mut bfres_cursor).unwrap();
                            println!("            {} points", lod.nb_points);
                            println!("            {} visibility groups", lod.nb_visibility_groups);
                        }
                    }
                }
            }
        }
        if let Some(a) = bfres_file.sub_file_index_groups.texture_data {
            println!("{} FTEX sub-files", a.entries.len());
            for ftex_entry in a.entries {
                println!(
                    "--- {} @ 0x{:x}",
                    ftex_entry.get_name(&mut bfres_cursor).unwrap(),
                    ftex_entry.data_pointer.get_abs_pos().unwrap()
                );
                let ftex = ftex_entry.get_data(&mut bfres_cursor).unwrap();
                println!(
                    "    Resolution: {} x {}",
                    ftex.header.texture_width, ftex.header.texture_height
                );
                println!("    Dimension: {}", ftex.header.dimension);
                println!("    AA Mode: {}", ftex.header.aa_mode);
                println!("    Usage: {}", ftex.header.usage);
                println!(
                    "    Tile Mode: {}, 0x{:X}",
                    ftex.header.tile_mode, ftex.header.tile_mode as u32
                );
                println!("    Component Selector: {}", ftex.header.component_selector);
                println!(
                    "    Texture Format: {}, 0x{:X}",
                    ftex.header.texture_format, ftex.header.texture_format as u32
                );
                println!("    Alignment: {}", ftex.header.alignment);
                println!("    Mipmaps: {}", ftex.header.nb_mipmaps);
                println!("    Array length: {}", ftex.header.array_length);
                println!("    Number of slices: {}", ftex.header.nb_slices);
                println!("    Depth: {}", ftex.header.texture_depth);
                println!("    Swizzle Value: {}", ftex.header.swizzle_value);
                println!("    Pitch: {}", ftex.header.pitch);
                println!(
                    "    Data Offset: {}",
                    ftex.header.data_offset.get_abs_pos().unwrap()
                );
                println!("    Data Length: {}", ftex.header.data_length);
            }
        }
        if let Some(a) = bfres_file.sub_file_index_groups.embedded_file {
            println!("{} Embedded sub-files", a.entries.len());
            for embedded_entry in a.entries {
                println!(
                    "--- {} @ 0x{:x}",
                    embedded_entry.get_name(&mut bfres_cursor).unwrap(),
                    embedded_entry.data_pointer.get_abs_pos().unwrap()
                );
                let embedded = embedded_entry.get_data(&mut bfres_cursor).unwrap();
                println!("    File @ 0x{:x}", embedded.offset.get_abs_pos().unwrap());
                println!(
                    "    {} byte{} long",
                    embedded.length,
                    if embedded.length == 1 { "" } else { "s" }
                );
            }
        }
    }
}
