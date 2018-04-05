use std::error::Error;
use std::io::{Read, Seek};
use util::Importable;

pub struct FVIS {}

impl Importable for FVIS {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FVIS, Box<Error>> {
        Ok(FVIS {})
    }
}
