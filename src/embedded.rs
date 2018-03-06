use std::io::{Read, Seek};
use std::error::Error;
use Importable;

pub struct Embedded {

}

impl Importable for Embedded {
    fn import<R: Read + Seek>(_reader: &mut R) -> Result<Embedded, Box<Error>> {
        Ok(Embedded {})
    }
}