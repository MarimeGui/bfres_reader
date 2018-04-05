use std::error::Error;
use std::io::{Read, Seek};
use util::Importable;

pub struct FSHA {}

impl Importable for FSHA {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FSHA, Box<Error>> {
        Ok(FSHA {})
    }
}
