use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek};
use std::fmt;
use Importable;
use util::Pointer;
use error::{check_magic_number, UnrecognizedFTEXDimension, UnrecognizedFTEXTileMode, UnrecognizedFTEXAAMode, UnrecognizedFTEXComponentSelectorChannel};

pub struct FTEX {
    pub header: FTEXHeader
}

pub struct FTEXHeader {
    pub dimension: FTEXDimension,
    pub texture_width: u32,
    pub texture_height: u32,
    pub texture_depth: u32,
    pub nb_mipmaps: u32,
    pub texture_format: u32,
    pub aa_mode: FTEXAAMode,
    pub usage: FTEXUsage,
    pub data_length: u32,
    pub mipmaps_data_length: u32,
    pub tile_mode: FTEXTileMode,
    pub swizzle_value: u32,
    pub alignment: u32,
    pub pitch: u32,
    pub mipmap_offsets: [u32; 13],
    pub first_mipmap: u32,
    pub nb_slices: u32,
    pub component_selector: FTEXComponentSelector,
    pub texture_registers: [u32; 5],
    pub array_length: u32,
    pub file_name_offset: Pointer,
    pub file_path_offset: Pointer,
    pub data_offset: Pointer,
    pub mipmap_offset: Pointer,
    pub user_data_index_group_offset: Pointer,
    pub user_data_entry_count: u16
}

pub enum FTEXDimension {
    OneD,
    TwoD,
    ThreeD,
    Cube,
    OneDArray,
    TwoDArray,
    TwoDMSAA,
    TwoDMSAAArray
}

pub enum FTEXAAMode {
    OneTime,
    TwoTimes,
    FourTimes,
    EightTimes
}

pub struct FTEXUsage {
    texture: bool,
    color_buffer: bool,
    depth_buffer: bool,
    scan_buffer: bool,
    ftv: bool
}

pub enum FTEXTileMode {
    Default,
    LinearSpecial,
    LinearAligned,
    OneDTiledThin1,
    OneDTiledThick,
    TwoDTiledThin1,
    TwoDTiledThin2,
    TwoDTiledThin4,
    TwoDTiledThick,
    TwoBTiledThin1,
    TwoBTiledThin2,
    TwoBTiledThin4,
    TwoBTiledThick,
    ThreeDTiledThin1,
    ThreeDTiledThick,
    ThreeBTiledThin1,
    ThreeBTiledThick
}

pub struct FTEXComponentSelector {
    composition: [FTEXComponentSelectorChannel; 4]
}

pub enum FTEXComponentSelectorChannel {
    Red,
    Green,
    Blue,
    Alpha,
    Zero,
    One
}

impl Importable for FTEX {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEX, Box<Error>> {
        let header = FTEXHeader::import(reader)?;
        Ok(FTEX {
            header
        })
    }
}

impl Importable for FTEXHeader {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEXHeader, Box<Error>> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        check_magic_number(magic_number, [b'F', b'T', b'E', b'X'])?;
        let dimension = FTEXDimension::import(reader)?;
        let texture_width = reader.read_be_to_u32()?;
        let texture_height = reader.read_be_to_u32()?;
        let texture_depth = reader.read_be_to_u32()?;
        let nb_mipmaps = reader.read_be_to_u32()?;
        let texture_format = reader.read_be_to_u32()?;
        let aa_mode = FTEXAAMode::import(reader)?;
        let usage = FTEXUsage::import(reader)?;
        let data_length = reader.read_be_to_u32()?;
        let data_pointer = reader.read_be_to_u32()?;
        assert_eq!(data_pointer, 0, "Data pointer is always 0 in files");
        let mipmaps_data_length = reader.read_be_to_u32()?;
        let mipmaps_pointer = reader.read_be_to_u32()?;
        assert_eq!(mipmaps_pointer, 0, "Mipmaps pointer is always 0 in files");
        let tile_mode = FTEXTileMode::import(reader)?;
        let swizzle_value = reader.read_be_to_u32()?;
        let alignment = reader.read_be_to_u32()?;
        let pitch = reader.read_be_to_u32()?;
        let mut mipmap_offsets: [u32; 13] = [0u32; 13];
        for data in &mut mipmap_offsets {
            *data = reader.read_be_to_u32()?;
        }
        let first_mipmap = reader.read_be_to_u32()?;
        let nb_mipmaps2 = reader.read_be_to_u32()?;
        assert_eq!(nb_mipmaps, nb_mipmaps2, "The two number of mipmaps are not equal");
        let first_slice = reader.read_be_to_u32()?;
        assert_eq!(first_slice, 0, "First slice is always 0");
        let nb_slices = reader.read_be_to_u32()?;
        let component_selector = FTEXComponentSelector::import(reader)?;
        let mut texture_registers: [u32; 5] = [0u32; 5];
        for data in &mut texture_registers {
            *data = reader.read_be_to_u32()?;
        }
        let texture_handle = reader.read_be_to_u32()?;
        assert_eq!(texture_handle, 0, "Texture handle is always 0 in files");
        let array_length = reader.read_be_to_u32()?;
        let file_name_offset = Pointer::read_new_rel_i32_be(reader)?;
        let file_path_offset = Pointer::read_new_rel_i32_be(reader)?;
        let data_offset = Pointer::read_new_rel_i32_be(reader)?;
        let mipmap_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_data_index_group_offset = Pointer::read_new_rel_i32_be(reader)?;
        let user_data_entry_count = reader.read_be_to_u16()?;
        Ok(FTEXHeader {
            dimension,
            texture_width,
            texture_height,
            texture_depth,
            nb_mipmaps,
            texture_format,
            aa_mode,
            usage,
            data_length,
            mipmaps_data_length,
            tile_mode,
            swizzle_value,
            alignment,
            pitch,
            mipmap_offsets,
            first_mipmap,
            nb_slices,
            component_selector,
            texture_registers,
            array_length,
            file_name_offset,
            file_path_offset,
            data_offset,
            mipmap_offset,
            user_data_index_group_offset,
            user_data_entry_count
        })
    }
}

impl Importable for FTEXDimension {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEXDimension, Box<Error>> {
        let value = reader.read_be_to_u32()?;
        Ok(match value {
            0x000 => FTEXDimension::OneD,
            0x001 => FTEXDimension::TwoD,
            0x002 => FTEXDimension::ThreeD,
            0x003 => FTEXDimension::Cube,
            0x004 => FTEXDimension::OneDArray,
            0x005 => FTEXDimension::TwoDArray,
            0x006 => FTEXDimension::TwoDMSAA,
            0x007 => FTEXDimension::TwoDMSAAArray,
            _ => return Err(Box::new(UnrecognizedFTEXDimension {value}))
        })
    }
}

impl fmt::Display for FTEXDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match *self {
            FTEXDimension::OneD      =>     "1D",
            FTEXDimension::TwoD      =>     "2D",
            FTEXDimension::ThreeD    =>     "3D",
            FTEXDimension::Cube      =>     "Cube",
            FTEXDimension::OneDArray =>     "1D Array",
            FTEXDimension::TwoDArray =>     "2D Array",
            FTEXDimension::TwoDMSAA  =>     "2D MSAA",
            FTEXDimension::TwoDMSAAArray => "2D MSAA Array",
        };
        write!(f, "{}", text)
    }
}

impl Importable for FTEXAAMode {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEXAAMode, Box<Error>> {
        let value = reader.read_be_to_u32()?;
        Ok(match value {
            0 => FTEXAAMode::OneTime,
            1 => FTEXAAMode::TwoTimes,
            2 => FTEXAAMode::FourTimes,
            3 => FTEXAAMode::EightTimes,
            _ => return Err(Box::new(UnrecognizedFTEXAAMode {value}))
        })
    }
}

impl fmt::Display for FTEXAAMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            FTEXAAMode::OneTime    => "1x",
            FTEXAAMode::TwoTimes   => "2x",
            FTEXAAMode::FourTimes  => "4x",
            FTEXAAMode::EightTimes => "8x"
        })
    }
}

impl Importable for FTEXUsage {
    fn import<R: Read +  Seek>(reader: &mut R) -> Result<FTEXUsage, Box<Error>> {
        let value = reader.read_be_to_u32()?;
        let texture = value & 1 == 1;
        let color_buffer = value & 2 == 2;
        let depth_buffer = value & 4 == 4;
        let scan_buffer = value & 8 == 8;
        let ftv = value & (1<<31) == (1<<31);
        Ok(FTEXUsage {
            texture,
            color_buffer,
            depth_buffer,
            scan_buffer,
            ftv
        })
    }
}

impl fmt::Display for FTEXUsage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = "".to_string();
        if self.texture {
            text += "Texture, ";
        };
        if self.color_buffer {
            text += "Color Buffer, ";
        };
        if self.depth_buffer {
            text += "Depth Buffer, ";
        };
        if self.scan_buffer {
            text += "Scan Buffer, ";
        };
        if self.ftv {
            text += "Final TV, ";
        };
        text.pop();
        text.pop();
        write!(f, "{}", text)
    }
}

impl Importable for FTEXTileMode {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEXTileMode, Box<Error>> {
        let value = reader.read_be_to_u32()?;
        Ok(match value {
            0x00 => FTEXTileMode::Default,
            0x10 => FTEXTileMode::LinearSpecial,
            0x01 => FTEXTileMode::LinearAligned,
            0x02 => FTEXTileMode::OneDTiledThin1,
            0x03 => FTEXTileMode::OneDTiledThick,
            0x04 => FTEXTileMode::TwoDTiledThin1,
            0x05 => FTEXTileMode::TwoDTiledThin2,
            0x06 => FTEXTileMode::TwoDTiledThin4,
            0x07 => FTEXTileMode::TwoDTiledThick,
            0x08 => FTEXTileMode::TwoBTiledThin1,
            0x09 => FTEXTileMode::TwoBTiledThin2,
            0x0A => FTEXTileMode::TwoBTiledThin4,
            0x0B => FTEXTileMode::TwoBTiledThick,
            0x0C => FTEXTileMode::ThreeDTiledThin1,
            0x0D => FTEXTileMode::ThreeDTiledThick,
            0x0E => FTEXTileMode::ThreeBTiledThin1,
            0x0F => FTEXTileMode::ThreeBTiledThick,
            _ => return Err(Box::new(UnrecognizedFTEXTileMode {value}))
        })
    }
}

impl fmt::Display for FTEXTileMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match *self {
            FTEXTileMode::Default => "Default",
            FTEXTileMode::LinearSpecial => "Linear Special",
            FTEXTileMode::LinearAligned => "Linear Aligned",
            FTEXTileMode::OneDTiledThin1 => "One D Tiled Thin 1",
            FTEXTileMode::OneDTiledThick => "One D Tiled Thick",
            FTEXTileMode::TwoDTiledThin1 => "Two D Tiled Thin 1",
            FTEXTileMode::TwoDTiledThin2 => "Two D Tiled Thin 2",
            FTEXTileMode::TwoDTiledThin4 => "Two D Tiled Thin 4",
            FTEXTileMode::TwoDTiledThick => "Two D Tiled Thick",
            FTEXTileMode::TwoBTiledThin1 => "Two B Tiled Thin 1",
            FTEXTileMode::TwoBTiledThin2 => "Two B Tiled Thin 2",
            FTEXTileMode::TwoBTiledThin4 => "Two B Tiled Thin 4",
            FTEXTileMode::TwoBTiledThick => "Two B Tiled Thick",
            FTEXTileMode::ThreeDTiledThin1 => "Three D Tiled Thin 1",
            FTEXTileMode::ThreeDTiledThick => "Three D Tiled Thick",
            FTEXTileMode::ThreeBTiledThin1 => "Three B Tiled Thin 1",
            FTEXTileMode::ThreeBTiledThick => "Three B Tiled Thick",
        };
        write!(f, "{}", text)
    }
}

impl Importable for FTEXComponentSelector {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEXComponentSelector, Box<Error>> {
        Ok(FTEXComponentSelector {
            composition: [
                FTEXComponentSelectorChannel::import(reader)?,
                FTEXComponentSelectorChannel::import(reader)?,
                FTEXComponentSelectorChannel::import(reader)?,
                FTEXComponentSelectorChannel::import(reader)?
            ]
        })
    }
}

impl fmt::Display for FTEXComponentSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}, {}", self.composition[0], self.composition[1], self.composition[2], self.composition[3])
    }
}

impl Importable for FTEXComponentSelectorChannel {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEXComponentSelectorChannel, Box<Error>> {
        let byte = reader.read_to_u8()?;
        Ok(match byte {
            0 => FTEXComponentSelectorChannel::Red,
            1 => FTEXComponentSelectorChannel::Green,
            2 => FTEXComponentSelectorChannel::Blue,
            3 => FTEXComponentSelectorChannel::Alpha,
            4 => FTEXComponentSelectorChannel::Zero,
            5 => FTEXComponentSelectorChannel::One,
            _ => return Err(Box::new(UnrecognizedFTEXComponentSelectorChannel {value: byte}))
        })
    }
}

impl fmt::Display for FTEXComponentSelectorChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            FTEXComponentSelectorChannel::Red => "Red",
            FTEXComponentSelectorChannel::Green => "Green",
            FTEXComponentSelectorChannel::Blue => "Blue",
            FTEXComponentSelectorChannel::Alpha => "Alpha",
            FTEXComponentSelectorChannel::Zero => "Always 0",
            FTEXComponentSelectorChannel::One => "Always 1"
        })
    }
}