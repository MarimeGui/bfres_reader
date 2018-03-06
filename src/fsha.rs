use std::io::{Read, Seek};
use std::error::Error;
use Importable;

pub struct FSHA {

}

impl Importable for FSHA {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FSHA, Box<Error>> {
        Ok(FSHA {})
    }
}