use Importable;
use std::error::Error;
use std::io::{Read, Seek};

pub struct FSHA {}

impl Importable for FSHA {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FSHA, Box<Error>> {
        Ok(FSHA {})
    }
}
