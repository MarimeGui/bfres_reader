use error::UnrecognizedFTEXComponentSelectorChannel;
use ez_io::ReadE;
use std::error::Error;
use std::fmt;
use std::io::{Read, Seek};
use util::Importable;

pub struct ComponentSelector {
    composition: [Channel; 4],
}

#[derive(Copy, Clone)]
pub enum Channel {
    Red = 0,
    Green = 1,
    Blue = 2,
    Alpha = 3,
    Zero = 4,
    One = 5,
}

impl Importable for ComponentSelector {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<ComponentSelector, Box<Error>> {
        Ok(ComponentSelector {
            composition: [
                Channel::import(reader)?,
                Channel::import(reader)?,
                Channel::import(reader)?,
                Channel::import(reader)?,
            ],
        })
    }
}

impl fmt::Display for ComponentSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.composition[0], self.composition[1], self.composition[2], self.composition[3]
        )
    }
}

impl Importable for Channel {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Channel, Box<Error>> {
        let byte = reader.read_to_u8()?;
        Ok(match byte {
            0 => Channel::Red,
            1 => Channel::Green,
            2 => Channel::Blue,
            3 => Channel::Alpha,
            4 => Channel::Zero,
            5 => Channel::One,
            _ => {
                return Err(Box::new(UnrecognizedFTEXComponentSelectorChannel {
                    value: byte,
                }))
            }
        })
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Channel::Red => "Red",
                Channel::Green => "Green",
                Channel::Blue => "Blue",
                Channel::Alpha => "Alpha",
                Channel::Zero => "Always 0",
                Channel::One => "Always 1",
            }
        )
    }
}
