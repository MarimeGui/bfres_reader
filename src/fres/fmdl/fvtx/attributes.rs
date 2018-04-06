use error::UnrecognizedValue;
use ez_io::ReadE;
use std::error::Error;
use std::fmt;
use std::io::SeekFrom;
use std::io::{Read, Seek};
use util::{Importable, Pointer};

pub struct Attributes {
    pub attribute_name_offset: Pointer,
    pub buffer_info_index: u8,
    pub buffer_offset: u16, // Unsure if points to Info or actual buffer
    pub format: AttributesFormats,
}

pub enum AttributesFormats {
    U8ToF32 = 0x000,
    TwoU8ToTwoF32 = 0x004,
    TwoU16ToTwoF32 = 0x007,
    FourU8ToFourF32 = 0x00A,
    U8ToU32 = 0x100,
    TwoU8ToTwoU32 = 0x104,
    FourU8ToFourU32 = 0x10A,
    I8ToF32 = 0x200,
    TwoI8ToF32 = 0x204,
    TwoI16ToTwoF32 = 0x207,
    FourI8ToFourF32 = 0x20A,
    ThreeI10toThreeF32 = 0x20B,
    I8 = 0x300,
    TwoI8 = 0x304,
    FourI8 = 0x30A,
    F32 = 0x806,
    TwoF16ToTwoF32 = 0x808,
    TwoF32 = 0x80D,
    FourF16ToFourF32 = 0x80F,
    ThreeF32 = 0x811,
    FourF32 = 0x813,
}

impl Importable for Attributes {
    fn import<R: Read + Seek>(reader: &mut R) -> Result<Attributes, Box<Error>> {
        let attribute_name_offset = Pointer::read_new_rel_i32_be(reader)?;
        let buffer_info_index = reader.read_to_u8()?;
        reader.seek(SeekFrom::Current(1))?;
        let buffer_offset = reader.read_be_to_u16()?;
        let format = match reader.read_be_to_u32()? {
            0x0000 => AttributesFormats::U8ToF32,
            0x0004 => AttributesFormats::TwoU8ToTwoF32,
            0x0007 => AttributesFormats::TwoU16ToTwoF32,
            0x000A => AttributesFormats::FourU8ToFourF32,
            0x0100 => AttributesFormats::U8ToU32,
            0x0104 => AttributesFormats::TwoU8ToTwoU32,
            0x010A => AttributesFormats::FourU8ToFourU32,
            0x0200 => AttributesFormats::I8ToF32,
            0x0204 => AttributesFormats::TwoI8ToF32,
            0x0207 => AttributesFormats::TwoI16ToTwoF32,
            0x020A => AttributesFormats::FourI8ToFourF32,
            0x020B => AttributesFormats::ThreeI10toThreeF32,
            0x0300 => AttributesFormats::I8,
            0x0304 => AttributesFormats::TwoI8,
            0x030A => AttributesFormats::FourI8,
            0x0806 => AttributesFormats::F32,
            0x0808 => AttributesFormats::TwoF16ToTwoF32,
            0x080D => AttributesFormats::TwoF32,
            0x080F => AttributesFormats::FourF16ToFourF32,
            0x0811 => AttributesFormats::ThreeF32,
            0x0813 => AttributesFormats::FourF32,
            x => {
                return Err(Box::new(UnrecognizedValue {
                    value: x,
                    enum_name: "AttributesFormats".to_string(),
                }))
            }
        };
        Ok(Attributes {
            attribute_name_offset,
            buffer_info_index,
            buffer_offset,
            format,
        })
    }
}

impl fmt::Display for AttributesFormats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match *self {
            AttributesFormats::U8ToF32 => "One u8 to one F32",
            AttributesFormats::TwoU8ToTwoF32 => "Two u8 to two f32",
            AttributesFormats::TwoU16ToTwoF32 => "Two u16 to two f32",
            AttributesFormats::FourU8ToFourF32 => "Four u8 to four f32",
            AttributesFormats::U8ToU32 => "One u8 to one u32",
            AttributesFormats::TwoU8ToTwoU32 => "Two u8 to two u32",
            AttributesFormats::FourU8ToFourU32 => "Four u8 to four u32",
            AttributesFormats::I8ToF32 => "One i8 to one f32",
            AttributesFormats::TwoI8ToF32 => "Two i8 to one f32",
            AttributesFormats::TwoI16ToTwoF32 => "Two i16 to two f32",
            AttributesFormats::FourI8ToFourF32 => "Four i8 to four f32",
            AttributesFormats::ThreeI10toThreeF32 => "Three i10 to three f32",
            AttributesFormats::I8 => "One i8",
            AttributesFormats::TwoI8 => "Two i8",
            AttributesFormats::FourI8 => "Four i8",
            AttributesFormats::F32 => "One f32",
            AttributesFormats::TwoF16ToTwoF32 => "Two f16 to two f32",
            AttributesFormats::TwoF32 => "Two f32",
            AttributesFormats::FourF16ToFourF32 => "Four f16 to four f32",
            AttributesFormats::ThreeF32 => "Three f32",
            AttributesFormats::FourF32 => "Four f32",
        };
        write!(f, "{}", text)
    }
}
