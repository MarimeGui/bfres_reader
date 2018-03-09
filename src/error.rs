use std::fmt;
use std::error::Error;

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
pub struct MissingFVTXAttributeFormat;

impl Error for MissingFVTXAttributeFormat {
    fn description(&self) -> &str {
        "A format value was not recognized"
    }
}

impl fmt::Display for MissingFVTXAttributeFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}

#[derive(Debug)]
pub struct MissingFSHPLODModelPrimitiveType;

impl Error for MissingFSHPLODModelPrimitiveType {
    fn description(&self) -> &str {
        "A primitive type value was not recognized"
    }
}

impl fmt::Display for MissingFSHPLODModelPrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}

#[derive(Debug)]
pub struct MissingFSHPLODModelIndexFormat;

impl Error for MissingFSHPLODModelIndexFormat {
    fn description(&self) -> &str {
        "A index format value was not recognized"
    }
}

impl fmt::Display for MissingFSHPLODModelIndexFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}

#[derive(Debug)]
pub struct WrongMagicNumber<T: PartialEq + Sized> {
    left: T,
    right: T
}

impl <T: PartialEq + Sized + fmt::Debug> Error for WrongMagicNumber<T> {
    fn description(&self) -> &str {
        "A Magic Number check Failed"
    }
}

impl <T: PartialEq + Sized + fmt::Debug> fmt::Display for WrongMagicNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect Magic Number: {:?} != {:?}", self.left, self.right)
    }
}

pub fn check_magic_number<T: PartialEq + Sized + fmt::Debug>(left: T, right: T) -> Result<(), WrongMagicNumber<T>> {
    if left != right {
        Err(WrongMagicNumber {
            left,
            right
        })
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct UnrecognizedFTEXDimension {
    pub value: u32
}

impl Error for UnrecognizedFTEXDimension {
    fn description(&self) -> &str {
        "The read value did not match anything"
    }
}

impl fmt::Display for UnrecognizedFTEXDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognized value: 0x{:x}", self.value)
    }
}

#[derive(Debug)]
pub struct UnrecognizedFTEXTileMode {
    pub value: u32
}

impl Error for UnrecognizedFTEXTileMode {
    fn description(&self) -> &str {
        "The read value did not match anything"
    }
}

impl fmt::Display for UnrecognizedFTEXTileMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognized value: 0x{:x}", self.value)
    }
}

#[derive(Debug)]
pub struct UnrecognizedFTEXAAMode {
    pub value: u32
}

impl Error for UnrecognizedFTEXAAMode {
    fn description(&self) -> &str {
        "The read value did not match anything"
    }
}

impl fmt::Display for UnrecognizedFTEXAAMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognized value: 0x{:x}", self.value)
    }
}