use std::io::{Read, Seek};
use std::error::Error;
use Importable;

pub struct FSCN {

}

impl Importable for FSCN {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FSCN, Box<Error>> {
        Ok(FSCN {})
    }
}