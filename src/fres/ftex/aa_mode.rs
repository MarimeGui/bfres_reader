use error::UnrecognizedValue;
use ez_io::ReadE;
use std::error::Error;
use std::fmt;
use std::io::{Read, Seek};
use util::Importable;

#[derive(Copy, Clone)]
pub enum AAMode {
    OneTime = 0x0,
    TwoTimes = 0x1,
    FourTimes = 0x2,
    EightTimes = 0x3,
}

impl Importable for AAMode {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<AAMode, Box<Error>> {
        Ok(match reader.read_be_to_u32()? {
            0 => AAMode::OneTime,
            1 => AAMode::TwoTimes,
            2 => AAMode::FourTimes,
            3 => AAMode::EightTimes,
            x => {
                return Err(Box::new(UnrecognizedValue {
                    value: x,
                    enum_name: "AAMode".to_string(),
                }))
            }
        })
    }
}

impl fmt::Display for AAMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                AAMode::OneTime => "1x",
                AAMode::TwoTimes => "2x",
                AAMode::FourTimes => "4x",
                AAMode::EightTimes => "8x",
            }
        )
    }
}
