extern crate bfres;
extern crate yaz0lib_rust;
extern crate ez_io;
extern crate half;

use half::f16;
use ez_io::ReadE;
use bfres::fmdl::{FVTXAttributesFormats, FSHPLODModelPrimitiveType, FSHPLODModelIndexFormat};
use bfres::Importable;
use bfres::fres::FRES;
use std::env;
use std::error::Error;
use std::io::{BufReader, Cursor, Read, Write, Seek, SeekFrom};
use std::mem::transmute;
use std::fs::File;
use std::path::Path;

trait FloatRead: Read {
    fn read_f16_to_f32(&mut self) -> Result<f32, Box<Error>> {
        let mut temp: [u8; 2] = [0; 2];
        self.read_exact(&mut temp)?;
        temp.reverse();
        Ok(f32::from(f16::from_bits(unsafe {
            transmute(temp)
        })))
    }

    fn read_f32(&mut self) -> Result<f32, Box<Error>> {
        let mut temp: [u8; 4] = [0; 4];
        self.read_exact(&mut temp)?;
        temp.reverse();
        unsafe {
            Ok(transmute(temp))
        }
    }
}

impl<R: Read> FloatRead for R {}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        let exec_name = args[0].to_string();
        println!("Usage: ./{} input_file output_folder", Path::new(&exec_name).file_name().unwrap().to_str().unwrap());
    } else if args.len() > 3 {
        println!("Please only give two arguments");
    } else {
        // Input the data
        let input_file = args[1].to_string();
        let output_folder = args[2].to_string();
        let mut input_file_reader = BufReader::new(File::open(&input_file).expect("Failed to open file for reading"));
        // Decompress
        println!("Decompressing...");
        let output = yaz0lib_rust::decompress(&mut input_file_reader).unwrap();
        println!("Decompressed !");
        // Make the cursor
        let mut bfres_cursor: Cursor<Vec<u8>> = Cursor::new(output);
        let bfres_cursor_ref = &mut bfres_cursor;
        let bfres_file = FRES::import(bfres_cursor_ref).expect("Failed to read FRES file properly");

        // If some Model sub-file exists
        if let Some(model_data_index_group) = bfres_file.sub_file_index_groups.model_data {

            // Go through all Models
            println!("{} model sub-files", model_data_index_group.entries.len());
            for model_entry in model_data_index_group.entries {
                // Load the FMDL
                let fmdl = model_entry.get_data(bfres_cursor_ref).expect("Failed to read FMDL Sub-File");

                // Create the vector that will store the vertices positions for all the FVTXes
                let mut vertices_positions_groups = Vec::with_capacity(fmdl.fvtx_array.entries.len());

                // Go through all the FVTX data
                println!("    {} FVTX", fmdl.fvtx_array.entries.len());
                for fvtx_entry in fmdl.fvtx_array.entries {
                    // Load the FVTX
                    let fvtx = fvtx_entry.get_data(bfres_cursor_ref).expect("Failed to read FVTX data");

                    // Create the vector that stores the positions for the vertices
                    let mut vertices_positions = Vec::with_capacity(fvtx.header.nb_vertices as usize);

                    // Go through all the attributes
                    for attributes_entry in fvtx.attributes_index_group.entries {
                        // Check if these attributes describes the position of the vertices
                        if attributes_entry.get_name(bfres_cursor_ref).expect("Failed to read FVTX attribute name") != "_p0" {
                            // If it is not, then we do not read the rest
                            break
                        }

                        // Load the attributes
                        let attributes = attributes_entry.get_data(bfres_cursor_ref).expect("Failed to read FVTX Attributes");

                        // Get the index of the FVTXBuffer these attributes are tied to
                        let buffer_index = usize::from(attributes.buffer_info_index);

                        // Get the buffer
                        let buffer = fvtx.buffer_info_array.entries[buffer_index].get_data(bfres_cursor_ref).expect("Failed to read FVTX Buffer");

                        // Get the buffer location in file
                        let buffer_location = buffer.data_offset.get_abs_pos().expect("Failed to calculate ");

                        // Locate the end of the buffer
                        let buffer_end = buffer_location + u64::from(buffer.size);

                        // Seek to the location of the buffer
                        buffer.data_offset.seek_abs_pos(bfres_cursor_ref).expect("Failed to seek to a FVTX Buffer");

                        // Read the position of the vertex differently depending on data types
                        match attributes.format {
                            FVTXAttributesFormats::ThreeF32 => {
                                let to_skip = i64::from(buffer.stride - 12);
                                while bfres_cursor_ref.seek(SeekFrom::Current(0)).unwrap() < buffer_end {
                                    vertices_positions.push([bfres_cursor_ref.read_f32().unwrap(), bfres_cursor_ref.read_f32().unwrap(), bfres_cursor_ref.read_f32().unwrap()]);
                                    bfres_cursor_ref.seek(SeekFrom::Current(to_skip)).unwrap();
                                }
                            },
                            FVTXAttributesFormats::FourF16ToFourF32 => {
                                let to_skip = i64::from((buffer.stride - 8) + 2);
                                while bfres_cursor_ref.seek(SeekFrom::Current(0)).unwrap() < buffer_end {
                                    vertices_positions.push([bfres_cursor_ref.read_f16_to_f32().unwrap(), bfres_cursor_ref.read_f16_to_f32().unwrap(), bfres_cursor_ref.read_f16_to_f32().unwrap()]);
                                    bfres_cursor_ref.seek(SeekFrom::Current(to_skip)).unwrap();
                                }
                            },
                            _ => unimplemented!()
                        }
                    }

                    // Push the retrieved positions of vertices to the main group
                    println!("        {} new vertices positions", vertices_positions.len());
                    vertices_positions_groups.push(vertices_positions);

                }

                // Go through all the FSHP data
                println!("    {} FSHP", fmdl.fshp_index_group.entries.len());
                for fshp_entry in fmdl.fshp_index_group.entries {
                    // Load the FSHP data
                    let fshp = fshp_entry.get_data(bfres_cursor_ref).expect("Failed to read FSHP data");

                    // Load the first LOD Model data (the highest quality one)
                    let lod_model = fshp.lod_model_array.entries[0].get_data(bfres_cursor_ref).expect("Failed to read FSHP LOD Model");

                    // Check if the primitive type is a triangle
                    match lod_model.primitive_type {
                        FSHPLODModelPrimitiveType::Triangles => {},
                        _ => break
                    }

                    // Check if the index type is Big Endian u16
                    match lod_model.index_format {
                        FSHPLODModelIndexFormat::U16BigEndian => {},
                        _ => break
                    }

                    // Get the FSHP name
                    let fshp_name = fshp_entry.get_name(bfres_cursor_ref).expect("Failed to get FSHP Entry name");

                    // Get the FVTX index for this FSHP
                    let fvtx_index = fshp.header.fvtx_index as usize;

                    // Retrieve the vertices positions
                    let vertices_positions = &vertices_positions_groups[fvtx_index];

                    // Make the vector that will hold the faces
                    let mut faces = Vec::new();

                    // Load the Buffer Info
                    let buffer_info = lod_model.get_direct_buffer_info(bfres_cursor_ref).expect("Failed to read an FSHP Buffer Info");

                    // Calculate the end of the buffer
                    let buffer_end = buffer_info.data_offset.get_abs_pos().expect("Failed to calculate absolute position for buffer") + u64::from(buffer_info.size);

                    // Go to the beginning of the buffer
                    buffer_info.data_offset.seek_abs_pos(bfres_cursor_ref).expect("Failed to seek to FSHP Buffer");

                    // Read the faces
                    while bfres_cursor_ref.seek(SeekFrom::Current(0)).unwrap() < buffer_end {
                        faces.push([bfres_cursor_ref.read_be_to_u16().unwrap(), bfres_cursor_ref.read_be_to_u16().unwrap(), bfres_cursor_ref.read_be_to_u16().unwrap()]);
                    }

                    println!("        {} new faces", faces.len());

                    // Export the OBJ
                    let mut obj_file = File::create(format!("{}/{}.obj", output_folder, fshp_name)).unwrap();
                    for vertex_position in vertices_positions {
                        let text: String = format!("v {} {} {}\n", vertex_position[0], vertex_position[1], vertex_position[2]);
                        obj_file.write_all(&text.into_bytes()).unwrap();
                    }
                    for face in &faces {
                        let text: String = format!("f {} {} {}\n", face[0] + 1, face[1] + 1, face[2] + 1);
                        obj_file.write_all(&text.into_bytes()).unwrap();
                    }
                }

            }

        } else {
            println!("No model data in this file !");
        }
    }
}