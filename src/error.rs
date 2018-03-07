use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::error::Error;

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
pub struct MissingFVTXAttributeFormat;

impl Error for MissingFVTXAttributeFormat {
    fn description(&self) -> &str {
        "A format value was not recognized"
    }
}

impl Display for MissingFVTXAttributeFormat {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Yea error lol")
    }
}