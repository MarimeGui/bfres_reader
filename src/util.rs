use std::error::Error;
use error::RelativePointerDataInvalid;

#[derive(Clone, Copy)]
pub struct RelativePointer {
    pub location: u64,
    pub points_to: i64
}

impl RelativePointer {
    pub fn absolute_position(&self) -> Result<u64, Box<Error>> {
        let temp: i64 = self.location as i64 + self.points_to;
        if temp < 0 {
            return Err(Box::new(RelativePointerDataInvalid {}));
        };
        Ok(temp as u64)
    }
}