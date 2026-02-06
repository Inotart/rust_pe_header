use std::fmt::{self, Debug};

pub mod optional_header32;
pub mod optional_header64;
#[repr(C)]
#[derive(Debug)]
pub enum OptionalHeader {
    OptionalHeader32(optional_header32::OptionalHeader32),
    OptionalHeader64(optional_header64::OptionalHeader64),
    None
}
impl std::fmt::Display for OptionalHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionalHeader::OptionalHeader32(optional_header32) => {
                optional_header32.fmt(f)
            },
            OptionalHeader::OptionalHeader64(optional_header64) => {
                optional_header64.fmt(f)
            },
            OptionalHeader::None => {write!(f,"None")}
        }
    }
}