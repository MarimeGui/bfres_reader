use error::UnrecognizedValue;
use ez_io::ReadE;
use std::error::Error;
use std::fmt;
use std::io::{Read, Seek};
use util::Importable;

#[derive(Copy, Clone)]
pub enum Format {
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

impl Importable for Format {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Format, Box<Error>> {
        Ok(match reader.read_be_to_u32()? {
            0x001 => Format::TcR8Unorm,
            0x101 => Format::TcR8Uint,
            0x201 => Format::TcR8Snorm,
            0x301 => Format::TcR8Sint,
            0x002 => Format::TR4G4Unorm,
            0x005 => Format::TcdR16Unorm,
            0x105 => Format::TcR16Uint,
            0x205 => Format::TcR16Snorm,
            0x305 => Format::TcR16Sint,
            0x806 => Format::TcR16Float,
            0x007 => Format::TcR8G8Unorm,
            0x107 => Format::TcR8G8Uint,
            0x207 => Format::TcR8G8Snorm,
            0x307 => Format::TcR8G8Sint,
            0x008 => Format::TcsR5G6B5Unorm,
            0x00a => Format::TcR5G5B5A1Unorm,
            0x00b => Format::TcR4G4B4A4Unorm,
            0x00c => Format::TcA1B5G5R5Unorm,
            0x10d => Format::TcR32Uint,
            0x30d => Format::TcR32Sint,
            0x80e => Format::TcdR32Float,
            0x00f => Format::TcR16G16Unorm,
            0x10f => Format::TcR16G16Uint,
            0x20f => Format::TcR16G16Snorm,
            0x30f => Format::TcR16G16Sint,
            0x810 => Format::TcR16G16Float,
            0x111 => Format::TX24G8Uint,
            0x811 => Format::DD24S8Float,
            0x816 => Format::TcR11G11B10Float,
            0x019 => Format::TcsR10G10B10A2Unorm,
            0x119 => Format::TcR10G10B10A2Uint,
            0x219 => Format::TcR10G10B10A2Snorm,
            0x319 => Format::TcR10G10B10A2Sint,
            0x01a => Format::TcsR8G8B8A8Unorm,
            0x11a => Format::TcR8G8B8A8Uint,
            0x21a => Format::TcR8G8B8A8Snorm,
            0x31a => Format::TcR8G8B8A8Sint,
            0x41a => Format::TcsR8G8B8A8Srgb,
            0x01b => Format::TcsA2B10G10R10Unorm,
            0x11b => Format::TcA2B10G10R10Uint,
            0x11c => Format::TX32G8UintX24,
            0x11d => Format::TcR32G32Uint,
            0x31d => Format::TcR32G32Sint,
            0x81e => Format::TcR32G32Float,
            0x01f => Format::TcR16G16B16A16Unorm,
            0x11f => Format::TcR16G16B16A16Uint,
            0x21f => Format::TcR16G16B16A16Snorm,
            0x31f => Format::TcR16G16B16A16Sint,
            0x820 => Format::TcR16G16B16A16Float,
            0x122 => Format::TcR32G32B32A32Uint,
            0x322 => Format::TcR32G32B32A32Sint,
            0x823 => Format::TcR32G32B32A32Float,
            0x031 => Format::TBc1Unorm,
            0x431 => Format::TBc1Srgb,
            0x032 => Format::TBc2Unorm,
            0x432 => Format::TBc2Srgb,
            0x033 => Format::TBc3Unorm,
            0x433 => Format::TBc3Srgb,
            0x034 => Format::TBc4Unorm,
            0x234 => Format::TBc4Snorm,
            0x035 => Format::TBc5Unorm,
            0x235 => Format::TBc5Snorm,
            0x081 => Format::TNv12Unorm,
            x => {
                return Err(Box::new(UnrecognizedValue {
                    value: x,
                    enum_name: "Format".to_string(),
                }))
            }
        })
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = format!("0x{:X}", self.clone() as u32);
        write!(
            f,
            "{}",
            match *self {
                Format::TcsR8G8B8A8Unorm => "R8 G8 B8 A8",
                Format::TBc1Unorm => "Block Compression 1",
                Format::TBc1Srgb => "Block Compression 1 SRGB",
                Format::TBc2Unorm => "Block Compression 2",
                Format::TBc2Srgb => "Block Compression 2 SRGB",
                Format::TBc3Unorm => "Block Compression 3",
                Format::TBc3Srgb => "Block Compression 3 SRGB",
                Format::TBc4Unorm => "Block Compression 4",
                Format::TBc4Snorm => "Block Compression 4 Signed",
                Format::TBc5Unorm => "Block Compression 5",
                Format::TBc5Snorm => "Block Compression Signed",
                Format::TNv12Unorm => "NV12 (video)",
                _ => val.as_str(),
            }
        )
    }
}

impl Format {
    pub fn is_block_compressed(&self) -> bool {
        match *self {
            Format::TBc1Unorm
            | Format::TBc1Srgb
            | Format::TBc2Unorm
            | Format::TBc2Srgb
            | Format::TBc3Unorm
            | Format::TBc3Srgb
            | Format::TBc4Unorm
            | Format::TBc4Snorm
            | Format::TBc5Unorm
            | Format::TBc5Snorm => true,
            _ => false,
        }
    }
}
