use std::error::Error;
use std::io::{Read, Seek};
use util::Importable;

pub struct FSKA {}

impl Importable for FSKA {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FSKA, Box<Error>> {
        Ok(FSKA {})
    }
}
