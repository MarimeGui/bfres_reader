use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RelativePointerDataInvalid;

impl Error for RelativePointerDataInvalid {
    fn description(&self) -> &str {
        "Self Explanatory"
    }
}

impl fmt::Display for RelativePointerDataInvalid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}

#[derive(Debug)]
pub struct UnrecognizedValue<T: fmt::Debug> {
    pub enum_name: String,
    pub value: T,
}

impl<T: fmt::Debug> Error for UnrecognizedValue<T> {
    fn description(&self) -> &str {
        "A read value from file did not match anything in en enum"
    }
}

impl<T: fmt::Debug> fmt::Display for UnrecognizedValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} did not match any value possible in {} enum",
            self.value, self.enum_name
        )
    }
}

#[derive(Debug)]
pub struct WrongMagicNumber<T: PartialEq + Sized> {
    left: T,
    right: T,
}

impl<T: PartialEq + Sized + fmt::Debug> Error for WrongMagicNumber<T> {
    fn description(&self) -> &str {
        "A Magic Number check Failed"
    }
}

impl<T: PartialEq + Sized + fmt::Debug> fmt::Display for WrongMagicNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Incorrect Magic Number: {:?} != {:?}",
            self.left, self.right
        )
    }
}

pub fn check_magic_number<T: PartialEq + Sized + fmt::Debug>(
    left: T,
    right: T,
) -> Result<(), WrongMagicNumber<T>> {
    if left != right {
        Err(WrongMagicNumber { left, right })
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct IncorrectHeaderLength {
    pub size: u16,
}

impl Error for IncorrectHeaderLength {
    fn description(&self) -> &str {
        "Read header length is not the expected size"
    }
}

impl fmt::Display for IncorrectHeaderLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong size: {}", self.size)
    }
}

#[derive(Debug)]
pub struct UserDataNotEmpty<T> {
    pub data: T,
    pub data_desc: String,
}

impl<T: fmt::Debug> Error for UserDataNotEmpty<T> {
    fn description(&self) -> &str {
        "Read header length is not the expected size"
    }
}

impl<T: fmt::Debug> fmt::Display for UserDataNotEmpty<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {:?}", self.data_desc, self.data)
    }
}

#[derive(Debug)]
pub struct IndexGroupTooLong {
    pub stopped_at: u64,
    pub expected_end: u64,
}

impl Error for IndexGroupTooLong {
    fn description(&self) -> &str {
        "Stopped reading an IndexGroup further than expected"
    }
}

impl fmt::Display for IndexGroupTooLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Stopped at 0x{:X}, expected to end at 0x{:X}",
            self.stopped_at, self.expected_end
        )
    }
}
