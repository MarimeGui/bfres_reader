use std::error::Error;
use std::io::{Read, Seek};
use util::Importable;

pub struct FTXP {}

impl Importable for FTXP {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<FTXP, Box<Error>> {
        Ok(FTXP {})
    }
}
