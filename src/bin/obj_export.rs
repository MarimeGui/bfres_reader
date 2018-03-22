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

struct OBJFile {
    groups: Vec<OBJGroup>
}

struct OBJGroup {
    vertices_positions: Vec<[f32; 3]>,
    vertices_texture_coordinates: Vec<[f32; 2]>,
    faces: Vec<[u16; 3]>,
    name: String
}

impl OBJFile {
    fn export(&self, file: &mut File) -> Result<(), Box<Error>> {
        let mut offset: u16 = 0;
        for group in &self.groups {
            file.write_all(&format!("o {}\n", group.name).into_bytes())?;
            for vertex_position in &group.vertices_positions {
                file.write_all(&format!("v {} {} {}\n",  vertex_position[0], vertex_position[1], vertex_position[2]).into_bytes())?;
            }
            for vertex_texture_coordinates in &group.vertices_texture_coordinates {
                file.write_all(&format!("vt {} {}\n",  vertex_texture_coordinates[0], vertex_texture_coordinates[1]).into_bytes())?;
            }
            file.write_all(&"s 1\n".to_string().into_bytes())?;
            for face in &group.faces {
                file.write_all(&format!("f {one}/{one} {two}/{two} {three}/{three}\n", one = face[0] + 1 + offset, two = face[1] + 1 + offset, three = face[2] + 1 + offset).into_bytes())?;
            }
            offset += group.vertices_positions.len() as u16;
        }
        Ok(())
    }
}

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

    fn read_3i10_to_3f32(&mut self) -> Result<[f32; 3], Box<Error>> {
        let value = self.read_be_to_u32()?;
        let mut value1 = ((value & 0b00111111111100000000000000000000u32) >> 20) as u16;
        let mut value2 = ((value & 0b00000000000011111111110000000000u32) >> 10) as u16;
        let mut value3 = ( value & 0b00000000000000000000001111111111u32) as u16;
        if value & 0b00100000000000000000000000000000u32 == 0b00100000000000000000000000000000u32 {  // If Value 1 is negative
            value1 += 0b1111110000000000u16;
        }
        let value1_int: i16 = unsafe {
            transmute(value1)
        };
        let value1_float = f32::from(value1_int) / 32768f32;
        if value & 0b00000000000010000000000000000000u32 == 0b00000000000010000000000000000000u32 {  // If Value 2 is negative
            value2 += 0b1111110000000000u16;
        }
        let value2_int: i16 = unsafe {
            transmute(value2)
        };
        let value2_float = f32::from(value2_int) / 32768f32;
        if value & 0b00000000000000000000001000000000u32 == 0b00000000000000000000001000000000u32 {  // If Value 3 is negative
            value3 += 0b1111110000000000u16;
        }
        let value3_int: i16 = unsafe {
            transmute(value3)
        };
        let value3_float = f32::from(value3_int) / 512f32;
        Ok([value1_float, value2_float, value3_float])
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
        // Get the first Magic Number to check for compression
        let mut input_file_buf_reader = BufReader::new(input_file_reader);
        let mut yaz_check_buffer = [0u8; 4];
        input_file_buf_reader.read_exact(&mut yaz_check_buffer).expect("Failed to read first Magic Number");
        input_file_buf_reader.seek(SeekFrom::Start(0)).expect("Failed to re-seek to beginning of the file");
        // Decompress if necessary and make the cursor
        let mut bfres_cursor = if yaz_check_buffer == [b'Y', b'a', b'z', b'0'] {
            Cursor::new(yaz0lib_rust::decompress(&mut input_file_buf_reader).expect("Failed to decompress"))
        } else {
            let mut bfres_data = Vec::new();
            input_file_buf_reader.read_to_end(&mut bfres_data).expect("Failed to read all data");
            Cursor::new(bfres_data)
        };
        let bfres_cursor_ref = &mut bfres_cursor;
        let bfres_file = FRES::import(bfres_cursor_ref).expect("Failed to read FRES file properly");

        // If some Model sub-file exists
        if let Some(model_data_index_group) = bfres_file.sub_file_index_groups.model_data {

            // Go through all Models
            println!("{} model sub-files", model_data_index_group.entries.len());
            for model_entry in model_data_index_group.entries {
                // Load the FMDL
                let fmdl = model_entry.get_data(bfres_cursor_ref).expect("Failed to read FMDL Sub-File");

                // Get the FMDL name
                let fmdl_name = model_entry.get_name(bfres_cursor_ref).unwrap();

                // Crate a new instance of OBJFile for this model
                let mut obj_file = OBJFile {
                    groups: Vec::new()
                };

                // Create the vector that will store the vertices positions for all the FVTXes
                let mut vertices_positions_groups = Vec::with_capacity(fmdl.fvtx_array.entries.len());

                // Create the vector for vertices texture coordinates
                let mut vertices_texture_coordinates_groups = Vec::with_capacity(fmdl.fvtx_array.entries.len());

                // Go through all the FVTX data
                println!("    {} FVTX", fmdl.fvtx_array.entries.len());
                for fvtx_entry in fmdl.fvtx_array.entries {
                    // Load the FVTX
                    let fvtx = fvtx_entry.get_data(bfres_cursor_ref).expect("Failed to read FVTX data");

                    // Go through all the attributes
                    for attributes_entry in fvtx.attributes_index_group.entries {
                        fn read_buffer_two<R: Read + Seek>(fmt: FVTXAttributesFormats, stride: u16, buffer_end: u64, reader: &mut R) -> Result<Vec<[f32; 2]>, Box<Error>> {
                            let mut vertices_data = Vec::new();

                            match fmt {
                                FVTXAttributesFormats::TwoU16ToTwoF32 => {
                                    let to_skip = i64::from(stride - 4);
                                    while reader.seek(SeekFrom::Current(0)).unwrap() < buffer_end {
                                        vertices_data.push([f32::from(reader.read_be_to_u16().unwrap()) / 65536f32, f32::from(reader.read_be_to_u16().unwrap()) / 65536f32]);
                                        reader.seek(SeekFrom::Current(to_skip)).unwrap();
                                    }
                                },
                                FVTXAttributesFormats::TwoF16ToTwoF32 => {
                                    let to_skip = i64::from(stride - 4);
                                    while reader.seek(SeekFrom::Current(0)).unwrap() < buffer_end {
                                        vertices_data.push([reader.read_f16_to_f32()?, reader.read_f16_to_f32()?]);
                                        reader.seek(SeekFrom::Current(to_skip)).unwrap();
                                    }
                                },
                                FVTXAttributesFormats::TwoU8ToTwoF32 => {
                                    let to_skip = i64::from(stride - 2);
                                    while reader.seek(SeekFrom::Current(0)).unwrap() < buffer_end {
                                        vertices_data.push([f32::from(reader.read_to_u8()?) / 255f32, f32::from(reader.read_to_u8()?) / 255f32]);
                                        reader.seek(SeekFrom::Current(to_skip)).unwrap();
                                    }
                                },
                                FVTXAttributesFormats::TwoI16ToTwoF32 => {
                                    let to_skip = i64::from(stride - 4);
                                    while reader.seek(SeekFrom::Current(0)).unwrap() < buffer_end {
                                        vertices_data.push([f32::from(reader.read_be_to_i16()?) / 32767f32, f32::from(reader.read_be_to_i16()?) / 32767f32]);
                                        reader.seek(SeekFrom::Current(to_skip)).unwrap();
                                    }
                                },
                                _ => unimplemented!()
                            }

                            Ok(vertices_data)

                        }

                        // Define a function for reading the buffer
                        fn read_buffer_three<R: Read + Seek>(fmt: FVTXAttributesFormats, stride: u16, buffer_end: u64, reader: &mut R) -> Result<Vec<[f32; 3]>, Box<Error>> {
                            // Create the vector that will hold the new data
                            let mut vertices_data = Vec::new();

                            // Read the position of the vertex differently depending on data types
                            match fmt {
                                FVTXAttributesFormats::ThreeF32 => {
                                    let to_skip = i64::from(stride - 12);
                                    while reader.seek(SeekFrom::Current(0)).unwrap() < buffer_end {
                                        vertices_data.push([reader.read_f32()?, reader.read_f32()?, reader.read_f32()?]);
                                        reader.seek(SeekFrom::Current(to_skip))?;
                                    }
                                },
                                FVTXAttributesFormats::FourF16ToFourF32 => {
                                    let to_skip = i64::from((stride - 8) + 2);
                                    while reader.seek(SeekFrom::Current(0)).unwrap() < buffer_end {
                                        vertices_data.push([reader.read_f16_to_f32()?, reader.read_f16_to_f32()?, reader.read_f16_to_f32()?]);
                                        reader.seek(SeekFrom::Current(to_skip)).unwrap();
                                    }
                                },
                                FVTXAttributesFormats::ThreeI10toThreeF32 => {
                                    let to_skip = i64::from(stride - 4);
                                    while reader.seek(SeekFrom::Current(0)).unwrap() < buffer_end {
                                        vertices_data.push(reader.read_3i10_to_3f32()?);
                                        reader.seek(SeekFrom::Current(to_skip)).unwrap();
                                    }
                                },
                                _ => unimplemented!()
                            }

                            Ok(vertices_data)

                        }

                        // Get the attribute name
                        let attribute_name = attributes_entry.get_name(bfres_cursor_ref).expect("Failed to read FVTX attribute name");

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

                        // Align to the data we want to read
                        bfres_cursor_ref.seek(SeekFrom::Current(i64::from(attributes.buffer_offset))).expect("Failed to seek");

                        // Check if these attributes describes the position of the vertices
                        match attribute_name.as_ref() {
                            "_p0" => {
                                // Read the buffer
                                let vertices_positions = read_buffer_three(attributes.format, buffer.stride, buffer_end, bfres_cursor_ref).unwrap();

                                // Push the retrieved positions of vertices to the main group
                                println!("        {} new vertices positions", vertices_positions.len());
                                vertices_positions_groups.push(vertices_positions);

                            },
                            "_u0" => {
                                // Create the vector that stores the texture_coordinates
                                let vertices_texture_coordinates = read_buffer_two(attributes.format, buffer.stride, buffer_end, bfres_cursor_ref).unwrap();

                                // Push the new texture_coordinates to the main group
                                println!("        {} new vertices texture_coordinates", vertices_texture_coordinates.len());
                                vertices_texture_coordinates_groups.push(vertices_texture_coordinates);
                            },
                            _ => {continue}
                        }
                    }

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
                        _ => continue
                    }

                    // Check if the index type is Big Endian u16
                    match lod_model.index_format {
                        FSHPLODModelIndexFormat::U16BigEndian => {},
                        _ => continue
                    }

                    // Get the FSHP name
                    let fshp_name = fshp_entry.get_name(bfres_cursor_ref).expect("Failed to get FSHP Entry name");

                    // Get the FVTX index for this FSHP
                    let fvtx_index = fshp.header.fvtx_index as usize;

                    // Retrieve the vertices positions
                    let vertices_positions = &vertices_positions_groups[fvtx_index];

                    // Retrieve the vertices texture_coordinates
                    let vertices_texture_coordinates = &vertices_texture_coordinates_groups[fvtx_index];

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

                    // Add a new OBJGroup for this model
                    let obj_group = OBJGroup {
                        vertices_positions: vertices_positions.clone(),
                        vertices_texture_coordinates: vertices_texture_coordinates.clone(),
                        faces,
                        name: fshp_name
                    };

                    // Add the new OBJGroup to OBJFile
                    obj_file.groups.push(obj_group);

                }

                // Create the output File object
                let mut obj_file_writer = File::create(format!("{}/{}.obj", output_folder, fmdl_name)).unwrap();

                // Export the OBJ file
                obj_file.export(&mut obj_file_writer).unwrap();
            }

        } else {
            println!("No model data in this file !");
        }
    }
}