use error::UnrecognizedFTEXDimension;
use ez_io::ReadE;
use std::error::Error;
use std::fmt;
use std::io::{Read, Seek};
use util::Importable;

#[derive(Copy, Clone)]
pub enum Dimension {
    OneD = 0x0,
    TwoD = 0x1,
    ThreeD = 0x2,
    Cube = 0x3,
    OneDArray = 0x4,
    TwoDArray = 0x5,
    TwoDMSAA = 0x6,
    TwoDMSAAArray = 0x7,
}

impl Importable for Dimension {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Dimension, Box<Error>> {
        let value = reader.read_be_to_u32()?;
        Ok(match value {
            0x000 => Dimension::OneD,
            0x001 => Dimension::TwoD,
            0x002 => Dimension::ThreeD,
            0x003 => Dimension::Cube,
            0x004 => Dimension::OneDArray,
            0x005 => Dimension::TwoDArray,
            0x006 => Dimension::TwoDMSAA,
            0x007 => Dimension::TwoDMSAAArray,
            _ => return Err(Box::new(UnrecognizedFTEXDimension { value })),
        })
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match *self {
            Dimension::OneD => "1D",
            Dimension::TwoD => "2D",
            Dimension::ThreeD => "3D",
            Dimension::Cube => "Cube",
            Dimension::OneDArray => "1D Array",
            Dimension::TwoDArray => "2D Array",
            Dimension::TwoDMSAA => "2D MSAA",
            Dimension::TwoDMSAAArray => "2D MSAA Array",
        };
        write!(f, "{}", text)
    }
}
