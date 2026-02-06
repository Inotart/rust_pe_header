use crate::pe::structs::{DWord, Word};

#[repr(C)]
#[derive(Debug)]
pub struct OptionalHeader32 {
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
    pub data_directory:Vec<crate::pe::structs::data_directory::DataDirectory>,// 数据目录项(16)

}
