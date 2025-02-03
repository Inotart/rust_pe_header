use super::structs;


/*
解析文件中的 pe数据
输入类型:Vec[u8]
pe文件起始位置:usize
*/
pub fn decode(date: Vec<u8>,pe_start: usize) -> Result<structs::DosHeader, Box<dyn std::error::Error>> {
    Ok(structs::DosHeader {
    })
}
fn read_u16_by_vec_u8(raw_date: &Vec<u8>, start: usize) -> u16 {
    return u16::from_le_bytes([raw_date[start], raw_date[start + 1]]);
}
fn read_move_vec_u18_list_by_vec_u8(raw_date: &Vec<u8>, start: usize, len: usize) -> Vec<u16> {
    let mut result = Vec::new();
    for i in 0..len {
        result.push(read_u16_by_vec_u8(raw_date, start + i * 2));
    }
    return result;
}
