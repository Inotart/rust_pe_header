// Word 类型,这里单纯至少为了阅读理解，尽量保持于原版一致
pub type Word = u16;
// 结构体 PEHeader
#[repr(C)]
#[derive(Debug)]
pub struct PEHeader {

}

pub fn word_to_string(word: Word) -> Result<String, Box<dyn std::error::Error>> {
    let vec_u8: Vec<u8> = word.to_le_bytes().to_vec();
    Ok(String::from_utf8(vec_u8)?)
}
