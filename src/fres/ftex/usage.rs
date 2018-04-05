use ez_io::ReadE;
use std::error::Error;
use std::fmt;
use std::io::{Read, Seek};
use util::Importable;

pub struct Usage {
    texture: bool,
    color_buffer: bool,
    depth_buffer: bool,
    scan_buffer: bool,
    ftv: bool,
}

impl Importable for Usage {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Usage, Box<Error>> {
        let value = reader.read_be_to_u32()?;
        let texture = value & 1 == 1;
        let color_buffer = value & 2 == 2;
        let depth_buffer = value & 4 == 4;
        let scan_buffer = value & 8 == 8;
        let ftv = value & (1 << 31) == (1 << 31);
        Ok(Usage {
            texture,
            color_buffer,
            depth_buffer,
            scan_buffer,
            ftv,
        })
    }
}

impl fmt::Display for Usage {
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
