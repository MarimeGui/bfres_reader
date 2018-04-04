use Importable;
use std::error::Error;
use std::io::{Read, Seek};

pub struct FSHU {}

impl Importable for FSHU {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FSHU, Box<Error>> {
        Ok(FSHU {})
    }
}
