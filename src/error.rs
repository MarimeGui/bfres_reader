use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::error::Error;

#[derive(Debug)]
pub struct WrongMagicNumber;

impl Error for WrongMagicNumber {
    fn description(&self) -> &str {
        "Self Explanatory"
    }
}

impl Display for WrongMagicNumber {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Yea error lol")
    }
}

#[derive(Debug)]
pub struct RelativePointerDataInvalid;

impl Error for RelativePointerDataInvalid {
    fn description(&self) -> &str {
        "Self Explanatory"
    }
}

impl Display for RelativePointerDataInvalid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Yea error lol")
    }
}

#[derive(Debug)]
pub struct NoRuntime;

impl Error for NoRuntime {
    fn description(&self) -> &str {
        "Found some runtime data that was set; We only read files here"
    }
}

impl Display for NoRuntime {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Yea error lol")
    }
}