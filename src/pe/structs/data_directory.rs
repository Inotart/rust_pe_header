use crate::pe::structs::DWord;

#[repr(C)]
#[derive(Debug)]
pub struct DataDirectory {
    pub virtual_address:DWord,// 虚拟地址
    pub size:DWord,// 大小
}