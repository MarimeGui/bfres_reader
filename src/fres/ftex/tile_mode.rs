use error::UnrecognizedValue;
use ez_io::ReadE;
use std::error::Error;
use std::fmt;
use std::io::{Read, Seek};
use util::Importable;

#[derive(Copy, Clone)]
pub enum TileMode {
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

impl Importable for TileMode {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<TileMode, Box<Error>> {
        Ok(match reader.read_be_to_u32()? {
            0x00 => TileMode::Default,
            0x10 => TileMode::LinearSpecial,
            0x01 => TileMode::LinearAligned,
            0x02 => TileMode::OneDTiledThin1,
            0x03 => TileMode::OneDTiledThick,
            0x04 => TileMode::TwoDTiledThin1,
            0x05 => TileMode::TwoDTiledThin2,
            0x06 => TileMode::TwoDTiledThin4,
            0x07 => TileMode::TwoDTiledThick,
            0x08 => TileMode::TwoBTiledThin1,
            0x09 => TileMode::TwoBTiledThin2,
            0x0A => TileMode::TwoBTiledThin4,
            0x0B => TileMode::TwoBTiledThick,
            0x0C => TileMode::ThreeDTiledThin1,
            0x0D => TileMode::ThreeDTiledThick,
            0x0E => TileMode::ThreeBTiledThin1,
            0x0F => TileMode::ThreeBTiledThick,
            x => {
                return Err(Box::new(UnrecognizedValue {
                    value: x,
                    enum_name: "TileMode".to_string(),
                }))
            }
        })
    }
}

impl fmt::Display for TileMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match *self {
            TileMode::Default => "Default",
            TileMode::LinearSpecial => "Linear Special",
            TileMode::LinearAligned => "Linear Aligned",
            TileMode::OneDTiledThin1 => "One D Tiled Thin 1",
            TileMode::OneDTiledThick => "One D Tiled Thick",
            TileMode::TwoDTiledThin1 => "Two D Tiled Thin 1",
            TileMode::TwoDTiledThin2 => "Two D Tiled Thin 2",
            TileMode::TwoDTiledThin4 => "Two D Tiled Thin 4",
            TileMode::TwoDTiledThick => "Two D Tiled Thick",
            TileMode::TwoBTiledThin1 => "Two B Tiled Thin 1",
            TileMode::TwoBTiledThin2 => "Two B Tiled Thin 2",
            TileMode::TwoBTiledThin4 => "Two B Tiled Thin 4",
            TileMode::TwoBTiledThick => "Two B Tiled Thick",
            TileMode::ThreeDTiledThin1 => "Three D Tiled Thin 1",
            TileMode::ThreeDTiledThick => "Three D Tiled Thick",
            TileMode::ThreeBTiledThin1 => "Three B Tiled Thin 1",
            TileMode::ThreeBTiledThick => "Three B Tiled Thick",
        };
        write!(f, "{}", text)
    }
}

impl TileMode {
    pub fn is_bank_swapped(&self) -> bool {
        match *self {
            TileMode::TwoBTiledThin1
            | TileMode::TwoBTiledThin2
            | TileMode::TwoBTiledThin4
            | TileMode::TwoBTiledThick
            | TileMode::ThreeBTiledThin1
            | TileMode::ThreeBTiledThick => true,
            _ => false,
        }
    }
    pub fn is_thick(&self) -> bool {
        match *self {
            TileMode::TwoDTiledThick
            | TileMode::TwoBTiledThick
            | TileMode::ThreeDTiledThick
            | TileMode::ThreeBTiledThick => true,
            _ => false,
        }
    }
    pub fn get_aspect_ratio(&self) -> u8 {
        match *self {
            TileMode::TwoDTiledThin2 | TileMode::TwoBTiledThin2 => 2,
            TileMode::TwoDTiledThin4 | TileMode::TwoBTiledThin4 => 4,
            _ => 1,
        }
    }
    pub fn get_surface_thickness(&self) -> u8 {
        match *self {
            TileMode::OneDTiledThick
            | TileMode::TwoDTiledThick
            | TileMode::TwoBTiledThick
            | TileMode::ThreeDTiledThick
            | TileMode::ThreeBTiledThick => 4,
            TileMode::LinearSpecial => 8,
            _ => 1,
        }
    }
}
