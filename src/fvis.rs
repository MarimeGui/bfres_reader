use std::io::{Read, Seek};
use std::error::Error;
use Importable;

pub struct FVIS {

}

impl Importable for FVIS {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FVIS, Box<Error>> {
        Ok(FVIS {})
    }
}