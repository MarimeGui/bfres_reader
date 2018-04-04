use Importable;
use std::error::Error;
use std::io::{Read, Seek};

pub struct FSCN {}

impl Importable for FSCN {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FSCN, Box<Error>> {
        Ok(FSCN {})
    }
}
