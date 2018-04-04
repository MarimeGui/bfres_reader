use Importable;
use error::{check_magic_number, UnrecognizedFTEXAAMode, UnrecognizedFTEXComponentSelectorChannel,
            UnrecognizedFTEXDimension, UnrecognizedFTEXFormat, UnrecognizedFTEXTileMode,
            UserDataNotEmpty};
use ez_io::ReadE;
use std::error::Error;
use std::fmt;
use std::io::{Read, Seek};
use util::Pointer;

pub struct FTEX {
    pub header: FTEXHeader,
}

pub struct FTEXHeader {
    pub dimension: FTEXDimension,
    pub texture_width: u32,
    pub texture_height: u32,
    pub texture_depth: u32,
    pub nb_mipmaps: u32,
    pub texture_format: FTEXFormat,
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
    pub user_data_entry_count: u16,
}

#[derive(Copy, Clone)]
pub enum FTEXDimension {
    OneD = 0x0,
    TwoD = 0x1,
    ThreeD = 0x2,
    Cube = 0x3,
    OneDArray = 0x4,
    TwoDArray = 0x5,
    TwoDMSAA = 0x6,
    TwoDMSAAArray = 0x7,
}

#[derive(Copy, Clone)]
pub enum FTEXFormat {
    TcR8Unorm = 0x001,
    TcR8Uint = 0x101,
    TcR8Snorm = 0x201,
    TcR8Sint = 0x301,
    TR4G4Unorm = 0x002,
    TcdR16Unorm = 0x005,
    TcR16Uint = 0x105,
    TcR16Snorm = 0x205,
    TcR16Sint = 0x305,
    TcR16Float = 0x806,
    TcR8G8Unorm = 0x007,
    TcR8G8Uint = 0x107,
    TcR8G8Snorm = 0x207,
    TcR8G8Sint = 0x307,
    TcsR5G6B5Unorm = 0x008,
    TcR5G5B5A1Unorm = 0x00a,
    TcR4G4B4A4Unorm = 0x00b,
    TcA1B5G5R5Unorm = 0x00c,
    TcR32Uint = 0x10d,
    TcR32Sint = 0x30d,
    TcdR32Float = 0x80e,
    TcR16G16Unorm = 0x00f,
    TcR16G16Uint = 0x10f,
    TcR16G16Snorm = 0x20f,
    TcR16G16Sint = 0x30f,
    TcR16G16Float = 0x810,
    TX24G8Uint = 0x111,
    DD24S8Float = 0x811,
    TcR11G11B10Float = 0x816,
    TcsR10G10B10A2Unorm = 0x019,
    TcR10G10B10A2Uint = 0x119,
    TcR10G10B10A2Snorm = 0x219,
    TcR10G10B10A2Sint = 0x319,
    TcsR8G8B8A8Unorm = 0x01a,
    TcR8G8B8A8Uint = 0x11a,
    TcR8G8B8A8Snorm = 0x21a,
    TcR8G8B8A8Sint = 0x31a,
    TcsR8G8B8A8Srgb = 0x41a,
    TcsA2B10G10R10Unorm = 0x01b,
    TcA2B10G10R10Uint = 0x11b,
    TX32G8UintX24 = 0x11c,
    TcR32G32Uint = 0x11d,
    TcR32G32Sint = 0x31d,
    TcR32G32Float = 0x81e,
    TcR16G16B16A16Unorm = 0x01f,
    TcR16G16B16A16Uint = 0x11f,
    TcR16G16B16A16Snorm = 0x21f,
    TcR16G16B16A16Sint = 0x31f,
    TcR16G16B16A16Float = 0x820,
    TcR32G32B32A32Uint = 0x122,
    TcR32G32B32A32Sint = 0x322,
    TcR32G32B32A32Float = 0x823,
    TBc1Unorm = 0x031,
    TBc1Srgb = 0x431,
    TBc2Unorm = 0x032,
    TBc2Srgb = 0x432,
    TBc3Unorm = 0x033,
    TBc3Srgb = 0x433,
    TBc4Unorm = 0x034,
    TBc4Snorm = 0x234,
    TBc5Unorm = 0x035,
    TBc5Snorm = 0x235,
    TNv12Unorm = 0x081,
}

#[derive(Copy, Clone)]
pub enum FTEXAAMode {
    OneTime = 0x0,
    TwoTimes = 0x1,
    FourTimes = 0x2,
    EightTimes = 0x3,
}

pub struct FTEXUsage {
    texture: bool,
    color_buffer: bool,
    depth_buffer: bool,
    scan_buffer: bool,
    ftv: bool,
}

#[derive(Copy, Clone)]
pub enum FTEXTileMode {
    Default = 0x00,
    LinearSpecial = 0x10,
    LinearAligned = 0x01,
    OneDTiledThin1 = 0x02,
    OneDTiledThick = 0x03,
    TwoDTiledThin1 = 0x04,
    TwoDTiledThin2 = 0x05,
    TwoDTiledThin4 = 0x06,
    TwoDTiledThick = 0x07,
    TwoBTiledThin1 = 0x08,
    TwoBTiledThin2 = 0x09,
    TwoBTiledThin4 = 0x0A,
    TwoBTiledThick = 0x0B,
    ThreeDTiledThin1 = 0x0C,
    ThreeDTiledThick = 0x0D,
    ThreeBTiledThin1 = 0x0E,
    ThreeBTiledThick = 0x0F, // Missing 16 and 17 ?
}

pub struct FTEXComponentSelector {
    composition: [FTEXComponentSelectorChannel; 4],
}

#[derive(Copy, Clone)]
pub enum FTEXComponentSelectorChannel {
    Red = 0,
    Green = 1,
    Blue = 2,
    Alpha = 3,
    Zero = 4,
    One = 5,
}

impl Importable for FTEX {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEX, Box<Error>> {
        let header = FTEXHeader::import(reader)?;
        Ok(FTEX { header })
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
        let texture_format = FTEXFormat::import(reader)?;
        let aa_mode = FTEXAAMode::import(reader)?;
        let usage = FTEXUsage::import(reader)?;
        let data_length = reader.read_be_to_u32()?;
        let data_pointer = reader.read_be_to_u32()?;
        if data_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: data_pointer,
                data_desc: "Data pointer".to_string(),
            }));
        }
        let mipmaps_data_length = reader.read_be_to_u32()?;
        let mipmaps_pointer = reader.read_be_to_u32()?;
        if mipmaps_pointer != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: mipmaps_pointer,
                data_desc: "Mipmaps Pointer".to_string(),
            }));
        }
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
        assert_eq!(
            nb_mipmaps, nb_mipmaps2,
            "The two number of mipmaps are not equal"
        );
        let first_slice = reader.read_be_to_u32()?;
        assert_eq!(first_slice, 0, "First slice is always 0");
        let nb_slices = reader.read_be_to_u32()?;
        let component_selector = FTEXComponentSelector::import(reader)?;
        let mut texture_registers: [u32; 5] = [0u32; 5];
        for data in &mut texture_registers {
            *data = reader.read_be_to_u32()?;
        }
        let texture_handle = reader.read_be_to_u32()?;
        if texture_handle != 0 {
            return Err(Box::new(UserDataNotEmpty {
                data: texture_handle,
                data_desc: "Texture Handle".to_string(),
            }));
        }
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
            user_data_entry_count,
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
            _ => return Err(Box::new(UnrecognizedFTEXDimension { value })),
        })
    }
}

impl fmt::Display for FTEXDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match *self {
            FTEXDimension::OneD => "1D",
            FTEXDimension::TwoD => "2D",
            FTEXDimension::ThreeD => "3D",
            FTEXDimension::Cube => "Cube",
            FTEXDimension::OneDArray => "1D Array",
            FTEXDimension::TwoDArray => "2D Array",
            FTEXDimension::TwoDMSAA => "2D MSAA",
            FTEXDimension::TwoDMSAAArray => "2D MSAA Array",
        };
        write!(f, "{}", text)
    }
}

impl Importable for FTEXFormat {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEXFormat, Box<Error>> {
        let value = reader.read_be_to_u32()?;
        Ok(match value {
            0x001 => FTEXFormat::TcR8Unorm,
            0x101 => FTEXFormat::TcR8Uint,
            0x201 => FTEXFormat::TcR8Snorm,
            0x301 => FTEXFormat::TcR8Sint,
            0x002 => FTEXFormat::TR4G4Unorm,
            0x005 => FTEXFormat::TcdR16Unorm,
            0x105 => FTEXFormat::TcR16Uint,
            0x205 => FTEXFormat::TcR16Snorm,
            0x305 => FTEXFormat::TcR16Sint,
            0x806 => FTEXFormat::TcR16Float,
            0x007 => FTEXFormat::TcR8G8Unorm,
            0x107 => FTEXFormat::TcR8G8Uint,
            0x207 => FTEXFormat::TcR8G8Snorm,
            0x307 => FTEXFormat::TcR8G8Sint,
            0x008 => FTEXFormat::TcsR5G6B5Unorm,
            0x00a => FTEXFormat::TcR5G5B5A1Unorm,
            0x00b => FTEXFormat::TcR4G4B4A4Unorm,
            0x00c => FTEXFormat::TcA1B5G5R5Unorm,
            0x10d => FTEXFormat::TcR32Uint,
            0x30d => FTEXFormat::TcR32Sint,
            0x80e => FTEXFormat::TcdR32Float,
            0x00f => FTEXFormat::TcR16G16Unorm,
            0x10f => FTEXFormat::TcR16G16Uint,
            0x20f => FTEXFormat::TcR16G16Snorm,
            0x30f => FTEXFormat::TcR16G16Sint,
            0x810 => FTEXFormat::TcR16G16Float,
            0x111 => FTEXFormat::TX24G8Uint,
            0x811 => FTEXFormat::DD24S8Float,
            0x816 => FTEXFormat::TcR11G11B10Float,
            0x019 => FTEXFormat::TcsR10G10B10A2Unorm,
            0x119 => FTEXFormat::TcR10G10B10A2Uint,
            0x219 => FTEXFormat::TcR10G10B10A2Snorm,
            0x319 => FTEXFormat::TcR10G10B10A2Sint,
            0x01a => FTEXFormat::TcsR8G8B8A8Unorm,
            0x11a => FTEXFormat::TcR8G8B8A8Uint,
            0x21a => FTEXFormat::TcR8G8B8A8Snorm,
            0x31a => FTEXFormat::TcR8G8B8A8Sint,
            0x41a => FTEXFormat::TcsR8G8B8A8Srgb,
            0x01b => FTEXFormat::TcsA2B10G10R10Unorm,
            0x11b => FTEXFormat::TcA2B10G10R10Uint,
            0x11c => FTEXFormat::TX32G8UintX24,
            0x11d => FTEXFormat::TcR32G32Uint,
            0x31d => FTEXFormat::TcR32G32Sint,
            0x81e => FTEXFormat::TcR32G32Float,
            0x01f => FTEXFormat::TcR16G16B16A16Unorm,
            0x11f => FTEXFormat::TcR16G16B16A16Uint,
            0x21f => FTEXFormat::TcR16G16B16A16Snorm,
            0x31f => FTEXFormat::TcR16G16B16A16Sint,
            0x820 => FTEXFormat::TcR16G16B16A16Float,
            0x122 => FTEXFormat::TcR32G32B32A32Uint,
            0x322 => FTEXFormat::TcR32G32B32A32Sint,
            0x823 => FTEXFormat::TcR32G32B32A32Float,
            0x031 => FTEXFormat::TBc1Unorm,
            0x431 => FTEXFormat::TBc1Srgb,
            0x032 => FTEXFormat::TBc2Unorm,
            0x432 => FTEXFormat::TBc2Srgb,
            0x033 => FTEXFormat::TBc3Unorm,
            0x433 => FTEXFormat::TBc3Srgb,
            0x034 => FTEXFormat::TBc4Unorm,
            0x234 => FTEXFormat::TBc4Snorm,
            0x035 => FTEXFormat::TBc5Unorm,
            0x235 => FTEXFormat::TBc5Snorm,
            0x081 => FTEXFormat::TNv12Unorm,
            _ => return Err(Box::new(UnrecognizedFTEXFormat { value })),
        })
    }
}

impl fmt::Display for FTEXFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = format!("0x{:X}", self.clone() as u32);
        write!(
            f,
            "{}",
            match *self {
                FTEXFormat::TcsR8G8B8A8Unorm => "R8 G8 B8 A8",
                FTEXFormat::TBc1Unorm => "Block Compression 1",
                FTEXFormat::TBc1Srgb => "Block Compression 1 SRGB",
                FTEXFormat::TBc2Unorm => "Block Compression 2",
                FTEXFormat::TBc2Srgb => "Block Compression 2 SRGB",
                FTEXFormat::TBc3Unorm => "Block Compression 3",
                FTEXFormat::TBc3Srgb => "Block Compression 3 SRGB",
                FTEXFormat::TBc4Unorm => "Block Compression 4",
                FTEXFormat::TBc4Snorm => "Block Compression 4 Signed",
                FTEXFormat::TBc5Unorm => "Block Compression 5",
                FTEXFormat::TBc5Snorm => "Block Compression Signed",
                FTEXFormat::TNv12Unorm => "NV12 (video)",
                _ => val.as_str(),
            }
        )
    }
}

impl FTEXFormat {
    pub fn is_block_compressed(&self) -> bool {
        match *self {
            FTEXFormat::TBc1Unorm
            | FTEXFormat::TBc1Srgb
            | FTEXFormat::TBc2Unorm
            | FTEXFormat::TBc2Srgb
            | FTEXFormat::TBc3Unorm
            | FTEXFormat::TBc3Srgb
            | FTEXFormat::TBc4Unorm
            | FTEXFormat::TBc4Snorm
            | FTEXFormat::TBc5Unorm
            | FTEXFormat::TBc5Snorm => true,
            _ => false,
        }
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
            _ => return Err(Box::new(UnrecognizedFTEXAAMode { value })),
        })
    }
}

impl fmt::Display for FTEXAAMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                FTEXAAMode::OneTime => "1x",
                FTEXAAMode::TwoTimes => "2x",
                FTEXAAMode::FourTimes => "4x",
                FTEXAAMode::EightTimes => "8x",
            }
        )
    }
}

impl Importable for FTEXUsage {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEXUsage, Box<Error>> {
        let value = reader.read_be_to_u32()?;
        let texture = value & 1 == 1;
        let color_buffer = value & 2 == 2;
        let depth_buffer = value & 4 == 4;
        let scan_buffer = value & 8 == 8;
        let ftv = value & (1 << 31) == (1 << 31);
        Ok(FTEXUsage {
            texture,
            color_buffer,
            depth_buffer,
            scan_buffer,
            ftv,
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
            _ => return Err(Box::new(UnrecognizedFTEXTileMode { value })),
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

impl FTEXTileMode {
    pub fn is_bank_swapped(&self) -> bool {
        match *self {
            FTEXTileMode::TwoBTiledThin1
            | FTEXTileMode::TwoBTiledThin2
            | FTEXTileMode::TwoBTiledThin4
            | FTEXTileMode::TwoBTiledThick
            | FTEXTileMode::ThreeBTiledThin1
            | FTEXTileMode::ThreeBTiledThick => true,
            _ => false,
        }
    }
    pub fn is_thick(&self) -> bool {
        match *self {
            FTEXTileMode::TwoDTiledThick
            | FTEXTileMode::TwoBTiledThick
            | FTEXTileMode::ThreeDTiledThick
            | FTEXTileMode::ThreeBTiledThick => true,
            _ => false,
        }
    }
    pub fn get_aspect_ratio(&self) -> u8 {
        match *self {
            FTEXTileMode::TwoDTiledThin2 | FTEXTileMode::TwoBTiledThin2 => 2,
            FTEXTileMode::TwoDTiledThin4 | FTEXTileMode::TwoBTiledThin4 => 4,
            _ => 1,
        }
    }
    pub fn get_surface_thickness(&self) -> u8 {
        match *self {
            FTEXTileMode::OneDTiledThick
            | FTEXTileMode::TwoDTiledThick
            | FTEXTileMode::TwoBTiledThick
            | FTEXTileMode::ThreeDTiledThick
            | FTEXTileMode::ThreeBTiledThick => 4,
            FTEXTileMode::LinearSpecial => 8,
            _ => 1,
        }
    }
}

impl Importable for FTEXComponentSelector {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<FTEXComponentSelector, Box<Error>> {
        Ok(FTEXComponentSelector {
            composition: [
                FTEXComponentSelectorChannel::import(reader)?,
                FTEXComponentSelectorChannel::import(reader)?,
                FTEXComponentSelectorChannel::import(reader)?,
                FTEXComponentSelectorChannel::import(reader)?,
            ],
        })
    }
}

impl fmt::Display for FTEXComponentSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.composition[0], self.composition[1], self.composition[2], self.composition[3]
        )
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
            _ => {
                return Err(Box::new(UnrecognizedFTEXComponentSelectorChannel {
                    value: byte,
                }))
            }
        })
    }
}

impl fmt::Display for FTEXComponentSelectorChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                FTEXComponentSelectorChannel::Red => "Red",
                FTEXComponentSelectorChannel::Green => "Green",
                FTEXComponentSelectorChannel::Blue => "Blue",
                FTEXComponentSelectorChannel::Alpha => "Alpha",
                FTEXComponentSelectorChannel::Zero => "Always 0",
                FTEXComponentSelectorChannel::One => "Always 1",
            }
        )
    }
}
