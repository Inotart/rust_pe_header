use std::{fs::File, io::Read};
use rust_pe_header::pe;
fn main() {
    // 读取a.exe的内容
    let mut file = File::open("mpay.dll").expect("Failed to open file");
    let mut date = Vec::new();
    file.read_to_end(&mut date).expect("Failed to read file");
    // 读取a.exe的DOS头
    let dos_header = rust_dos_header::dos::decode(date.clone()).expect("Failed to decode DOS header");
    
    // 获取pe地址
    let pe_header = pe::decode(date.clone(), dos_header.e_lfanew as usize).expect("Failed to decode PE header");
    println!("{:#?}", pe_header);
    println!("{}",pe::print(pe_header).expect("1"))
}
