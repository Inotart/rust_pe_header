use std::fmt;
pub mod file_header;
pub mod optional_header;
pub mod data_directory;
// Word 类型,这里单纯至少为了阅读理解，尽量保持于原版一致
pub type Word = u16;
pub type DWord = u32;
// 结构体 PEHeader
#[repr(C)]
#[derive(Debug)]
pub struct PEHeader {
    pub signature:DWord,// PE文件标识:ASCII的"PE\0\0"
    pub file_header:file_header::FileHeader,
    pub optional_header:optional_header::OptionalHeader,
}
impl fmt::Display for PEHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PEHeader")
            .field("signature", &format_args!("{:08X}", self.signature))
            .field("file_header", &self.file_header)
            .field("optional_header", &self.optional_header)
            .finish()
    }
}



pub fn word_to_string(word: Word) -> Result<String, Box<dyn std::error::Error>> {
    let vec_u8: Vec<u8> = word.to_le_bytes().to_vec();
    Ok(String::from_utf8(vec_u8)?)
}
pub fn word_to_string_32(word: DWord) -> Result<String, Box<dyn std::error::Error>> {
    let vec_u8: Vec<u8> = word.to_le_bytes().to_vec();
    Ok(String::from_utf8(vec_u8)?)
}