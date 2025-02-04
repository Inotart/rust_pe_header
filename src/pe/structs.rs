// Word 类型,这里单纯至少为了阅读理解，尽量保持于原版一致
pub type Word = u16;
pub type DWord = u32;
// 结构体 PEHeader
#[repr(C)]
#[derive(Debug)]
pub struct PEHeader {
    pub signature:DWord,// PE文件标识:ASCII的"PE\0\0"
    pub file_header:FileHeader,
    pub optional_header:Option<OptionalHeader>,
    pub optional_header64:Option<OptionalHeader64>,

}
#[repr(C)]
#[derive(Debug)]
pub struct FileHeader {
    pub machine:Word,// 机器类型
    pub number_of_sections:Word,// 文件中的节区数量
    pub time_date_stamp:DWord,// 文件创建时间
    pub pointer_to_symbol_table:DWord,// 指向符号表的指针
    pub number_of_symbols:DWord,// 符号表中的符号数量
    pub size_of_optional_header:Word,// 可选头的大小
    pub characteristics:Word,// 文件属性

}
#[repr(C)]
#[derive(Debug)]
pub struct OptionalHeader {
    pub magic:Word,// 可选头魔数
    pub major_linker_version:u8,// 链接器主版本号
    pub minor_linker_version:u8,// 链接器次版本号
    pub size_of_code:DWord,// 代码节的大小
    pub size_of_initialized_data:DWord,// 已初始化数据节的大小
    pub size_of_uninitialized_data:DWord,// 未初始化数据节的大小
    pub address_of_entry_point:DWord,// 程序入口RVA
    pub base_of_code:DWord,// 代码节基址
    pub base_of_data:DWord,// 数据节基址
    pub image_base:DWord,// 镜像基址
    pub section_alignment:Word,// 节对齐大小
    pub file_alignment:Word,// 文件对齐大小
    pub major_operating_system_version:Word,// 操作系统主版本号
    pub minor_operating_system_version:Word,// 操作系统次版本号
    pub major_image_version:Word,// 镜像主版本号
    pub minor_image_version:Word,// 镜像次版本号
    pub major_subsystem_version:Word,// 子系统主版本号
    pub minor_subsystem_version:Word,// 子系统次版本号
    pub win32_version_value:DWord,// 保留
    pub size_of_image:DWord,// 镜像大小
    pub size_of_headers:DWord,// 头大小
    pub check_sum:DWord,// 校验和
    pub subsystem:Word,// 子系统
    pub dll_characteristics:Word,// DLL特性
    pub size_of_stack_reserve:DWord,// 栈保留大小
    pub size_of_stack_commit:DWord,// 栈提交大小
    pub size_of_heap_reserve:DWord,// 堆保留大小
    pub size_of_heap_commit:DWord,// 堆提交大小
    pub loader_flags:DWord,// 加载标志
    pub number_of_rva_and_sizes:DWord,// 数据目录项数量
    pub data_directory:Vec<DataDirectory>,// 数据目录项(16)

}
#[repr(C)]
#[derive(Debug)]
pub struct OptionalHeader64 {
    pub magic:Word,// 可选头魔数
    pub major_linker_version:u8,// 链接器主版本号
    pub minor_linker_version:u8,// 链接器次版本号
    pub size_of_code:DWord,// 代码节的大小
    pub size_of_initialized_data:DWord,// 已初始化数据节的大小
    pub size_of_uninitialized_data:DWord,// 未初始化数据节的大小
    pub address_of_entry_point:DWord,// 程序入口RVA
    pub base_of_code:DWord,// 代码节基址
    pub image_base:u64,// 镜像基址
    pub section_alignment:DWord,// 节对齐大小
    pub file_alignment:DWord,// 文件对齐大小
    pub major_operating_system_version:Word,// 操作系统主版本号
    pub minor_operating_system_version:Word,// 操作系统次版本号
    pub major_image_version:Word,// 镜像主版本号
    pub minor_image_version:Word,// 镜像次版本号
    pub major_subsystem_version:Word,// 子系统主版本号
    pub minor_subsystem_version:Word,// 子系统次版本号
    pub win32_version_value:DWord,// 保留
    pub size_of_image:DWord,// 镜像大小
    pub size_of_headers:DWord,// 头大小
    pub check_sum:DWord,// 校验和
    pub subsystem:Word,// 子系统
    pub dll_characteristics:Word,// DLL特性
    pub size_of_stack_reserve:u64,// 栈保留大小
    pub size_of_stack_commit:u64,// 栈提交大小
    pub size_of_heap_reserve:u64,// 堆保留大小
    pub size_of_heap_commit:u64,// 堆提交大小
    pub loader_flags:DWord,// 加载标志
    pub number_of_rva_and_sizes:DWord,// 数据目录项数量
    pub data_directory:Vec<DataDirectory>,// 数据目录项(16)
    
}
#[repr(C)]
#[derive(Debug)]
pub struct DataDirectory {
    pub virtual_address:DWord,// 虚拟地址
    pub size:DWord,// 大小
}

pub fn word_to_string(word: Word) -> Result<String, Box<dyn std::error::Error>> {
    let vec_u8: Vec<u8> = word.to_le_bytes().to_vec();
    Ok(String::from_utf8(vec_u8)?)
}
pub fn word_to_string_32(word: DWord) -> Result<String, Box<dyn std::error::Error>> {
    let vec_u8: Vec<u8> = word.to_le_bytes().to_vec();
    Ok(String::from_utf8(vec_u8)?)
}