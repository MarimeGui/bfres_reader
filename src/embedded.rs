use Importable;
use ez_io::ReadE;
use std::error::Error;
use std::io::{Read, Seek};
use util::Pointer;

pub struct Embedded {
    pub offset: Pointer,
    pub length: u32,
}

impl Importable for Embedded {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Embedded, Box<Error>> {
        let offset = Pointer::read_new_rel_i32_be(reader)?;
        let length = reader.read_be_to_u32()?;
        Ok(Embedded { offset, length })
    }
}
