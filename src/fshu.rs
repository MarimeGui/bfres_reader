use std::io::{Read, Seek};
use std::error::Error;
use Importable;

pub struct FSHU {

}

impl Importable for FSHU {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FSHU, Box<Error>> {
        Ok(FSHU {})
    }
}