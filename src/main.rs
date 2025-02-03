use std::{fs::File, io::Read};

fn main() {
    // 读取a.exe的内容
    let mut file = File::open("a.exe").expect("Failed to open file");
    let mut date = Vec::new();
    file.read_to_end(&mut date).expect("Failed to read file");
    // 读取a.exe的DOS头
    let dos_header = rust_dos_header::dos::decode(date).expect("Failed to decode DOS header");
    // 获取pe地址
    // let pe_header = pe::decode
    
}
