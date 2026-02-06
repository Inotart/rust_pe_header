use super::{structs};

/*
解析文件中的 pe数据
输入类型:Vec[u8]
pe文件起始位置:usize
*/
pub fn decode(
    date: Vec<u8>,
    pe_start: usize,
) -> Result<structs::PEHeader, Box<dyn std::error::Error>> {
    let mut pe_header = structs::PEHeader {
        signature: 0,
        file_header: structs::file_header::FileHeader {
            machine: 0,
            number_of_sections: 0,
            time_date_stamp: 0,
            pointer_to_symbol_table: 0,
            number_of_symbols: 0,
            size_of_optional_header: 0,
            characteristics: 0,
        },
        optional_header: structs::optional_header::OptionalHeader::None,
    };
    // 解析 signature
    pe_header.signature = read_u32_by_vec_u8(&date, pe_start);
    // 解析 file_header
    pe_header.file_header.machine = read_u16_by_vec_u8(&date, pe_start + 4);
    pe_header.file_header.number_of_sections = read_u16_by_vec_u8(&date, pe_start + 6);
    pe_header.file_header.time_date_stamp = read_u32_by_vec_u8(&date, pe_start + 8);
    pe_header.file_header.pointer_to_symbol_table = read_u32_by_vec_u8(&date, pe_start + 12);
    pe_header.file_header.number_of_symbols = read_u32_by_vec_u8(&date, pe_start + 16);
    pe_header.file_header.size_of_optional_header = read_u16_by_vec_u8(&date, pe_start + 20);
    pe_header.file_header.characteristics = read_u16_by_vec_u8(&date, pe_start + 22);
    // 解析 optional_header
    let magic = read_u16_by_vec_u8(&date, pe_start + 24);
    match magic {
        0x10b => {
            let mut optional_header = structs::optional_header::optional_header32::OptionalHeader32 {
                magic: 0,
                major_linker_version: 0,
                minor_linker_version: 0,
                size_of_code: 0,
                size_of_initialized_data: 0,
                size_of_uninitialized_data: 0,
                address_of_entry_point: 0,
                base_of_code: 0,
                base_of_data: 0,
                image_base: 0,
                section_alignment: 0,
                file_alignment: 0,
                major_operating_system_version: 0,
                minor_operating_system_version: 0,
                major_image_version: 0,
                minor_image_version: 0,
                major_subsystem_version: 0,
                minor_subsystem_version: 0,
                win32_version_value: 0,
                size_of_image: 0,
                size_of_headers: 0,
                check_sum: 0,
                subsystem: 0,
                dll_characteristics: 0,
                size_of_stack_reserve: 0,
                size_of_stack_commit: 0,
                size_of_heap_reserve: 0,
                size_of_heap_commit: 0,
                loader_flags: 0,
                number_of_rva_and_sizes: 0,
                data_directory: Vec::new(),
            };
            optional_header.magic = read_u16_by_vec_u8(&date, pe_start + 24);
            optional_header.major_linker_version = read_u8_by_vec_u8(&date, pe_start + 26);
            optional_header.minor_linker_version = read_u8_by_vec_u8(&date, pe_start + 27);
            optional_header.size_of_code = read_u32_by_vec_u8(&date, pe_start + 28);
            optional_header.size_of_initialized_data = read_u32_by_vec_u8(&date, pe_start + 32);
            optional_header.size_of_uninitialized_data = read_u32_by_vec_u8(&date, pe_start + 36);
            optional_header.address_of_entry_point = read_u32_by_vec_u8(&date, pe_start + 40);
            optional_header.base_of_code = read_u32_by_vec_u8(&date, pe_start + 44);
            optional_header.base_of_data = read_u32_by_vec_u8(&date, pe_start + 48);
            optional_header.image_base = read_u32_by_vec_u8(&date, pe_start + 52);
            optional_header.section_alignment = read_u16_by_vec_u8(&date, pe_start + 60-4);
            optional_header.file_alignment = read_u16_by_vec_u8(&date, pe_start + 64-4);
            optional_header.major_operating_system_version =
                read_u16_by_vec_u8(&date, pe_start + 68-4);
            optional_header.minor_operating_system_version =
                read_u16_by_vec_u8(&date, pe_start + 70-4);
            optional_header.major_image_version = read_u16_by_vec_u8(&date, pe_start + 72-4);
            optional_header.minor_image_version = read_u16_by_vec_u8(&date, pe_start + 74-4);
            optional_header.major_subsystem_version = read_u16_by_vec_u8(&date, pe_start + 76-4);
            optional_header.minor_subsystem_version = read_u16_by_vec_u8(&date, pe_start + 78-4);
            optional_header.win32_version_value = read_u32_by_vec_u8(&date, pe_start + 80-4);
            optional_header.size_of_image = read_u32_by_vec_u8(&date, pe_start + 84-4);
            optional_header.size_of_headers = read_u32_by_vec_u8(&date, pe_start + 88-4);
            optional_header.check_sum = read_u32_by_vec_u8(&date, pe_start + 92-4);
            optional_header.subsystem = read_u16_by_vec_u8(&date, pe_start + 96-4);
            optional_header.dll_characteristics = read_u16_by_vec_u8(&date, pe_start + 98-4);
            optional_header.size_of_stack_reserve = read_u32_by_vec_u8(&date, pe_start + 100-4);
            optional_header.size_of_stack_commit = read_u32_by_vec_u8(&date, pe_start + 108-4);
            optional_header.size_of_heap_reserve = read_u32_by_vec_u8(&date, pe_start + 116-4);
            optional_header.size_of_heap_commit = read_u32_by_vec_u8(&date, pe_start + 124-4);
            optional_header.loader_flags = read_u32_by_vec_u8(&date, pe_start + 132-4);
            optional_header.number_of_rva_and_sizes = read_u32_by_vec_u8(&date, pe_start + 136-4);
            // 解析 data_directory
            let mut data_directory: Vec<structs::data_directory::DataDirectory> = Vec::new();
            for ie in 0..16 {
                let mut data_directory_item = structs::data_directory::DataDirectory {
                    virtual_address: 0,
                    size: 0,
                };
                let i = ie as usize;
                data_directory_item.virtual_address =
                    read_u32_by_vec_u8(&date, pe_start + 140 -4 + i * 8);
                data_directory_item.size = read_u32_by_vec_u8(&date, pe_start + 144 -4+ i * 8);
                data_directory.push(data_directory_item);
            }
            optional_header.data_directory = data_directory;
            pe_header.optional_header = structs::optional_header::OptionalHeader::OptionalHeader32(optional_header);
        }
        0x20b => {
            let mut optional_header64 = structs::optional_header::optional_header64::OptionalHeader64 {
                magic: 0,
                major_linker_version: 0,
                minor_linker_version: 0,
                size_of_code: 0,
                size_of_initialized_data: 0,
                size_of_uninitialized_data: 0,
                address_of_entry_point: 0,
                base_of_code: 0,
                image_base: 0,
                section_alignment: 0,
                file_alignment: 0,
                major_operating_system_version: 0,
                minor_operating_system_version: 0,
                major_image_version: 0,
                minor_image_version: 0,
                major_subsystem_version: 0,
                minor_subsystem_version: 0,
                win32_version_value: 0,
                size_of_image: 0,
                size_of_headers: 0,
                check_sum: 0,
                subsystem: 0,
                dll_characteristics: 0,
                size_of_stack_reserve: 0,
                size_of_stack_commit: 0,
                size_of_heap_reserve: 0,
                size_of_heap_commit: 0,
                loader_flags: 0,
                number_of_rva_and_sizes: 0,
                data_directory: Vec::new(),
            };
            optional_header64.magic = read_u16_by_vec_u8(&date, pe_start + 24);
            optional_header64.major_linker_version = read_u8_by_vec_u8(&date, pe_start + 26);
            optional_header64.minor_linker_version = read_u8_by_vec_u8(&date, pe_start + 27);
            optional_header64.size_of_code = read_u32_by_vec_u8(&date, pe_start + 28);
            optional_header64.size_of_initialized_data = read_u32_by_vec_u8(&date, pe_start + 32);
            optional_header64.size_of_uninitialized_data = read_u32_by_vec_u8(&date, pe_start + 36);
            optional_header64.address_of_entry_point = read_u32_by_vec_u8(&date, pe_start + 40);
            optional_header64.base_of_code = read_u32_by_vec_u8(&date, pe_start + 44);
            optional_header64.image_base = read_u64_by_vec_u8(&date, pe_start + 48);
            optional_header64.section_alignment = read_u32_by_vec_u8(&date, pe_start + 56);
            optional_header64.file_alignment = read_u32_by_vec_u8(&date, pe_start + 52+8);
            optional_header64.major_operating_system_version =
                read_u16_by_vec_u8(&date, pe_start + 56+8);
            optional_header64.minor_operating_system_version =
                read_u16_by_vec_u8(&date, pe_start + 58+8);
            optional_header64.major_image_version = read_u16_by_vec_u8(&date, pe_start + 60+8);
            optional_header64.minor_image_version = read_u16_by_vec_u8(&date, pe_start + 62+8);
            optional_header64.major_subsystem_version = read_u16_by_vec_u8(&date, pe_start + 64+8);
            optional_header64.minor_subsystem_version = read_u16_by_vec_u8(&date, pe_start + 66+8);
            optional_header64.win32_version_value = read_u32_by_vec_u8(&date, pe_start + 68+8);
            optional_header64.size_of_image = read_u32_by_vec_u8(&date, pe_start + 72+8);
            optional_header64.size_of_headers = read_u32_by_vec_u8(&date, pe_start + 76+8);
            optional_header64.check_sum = read_u32_by_vec_u8(&date, pe_start + 80+8);
            optional_header64.subsystem = read_u16_by_vec_u8(&date, pe_start + 84+8);
            optional_header64.dll_characteristics = read_u16_by_vec_u8(&date, pe_start + 86+8);
            optional_header64.size_of_stack_reserve = read_u64_by_vec_u8(&date, pe_start + 88+8);
            optional_header64.size_of_stack_commit = read_u64_by_vec_u8(&date, pe_start + 96+8);
            optional_header64.size_of_heap_reserve = read_u64_by_vec_u8(&date, pe_start + 104+8);
            optional_header64.size_of_heap_commit = read_u64_by_vec_u8(&date, pe_start + 112+8);
            optional_header64.loader_flags = read_u32_by_vec_u8(&date, pe_start + 120+8);
            optional_header64.number_of_rva_and_sizes = read_u32_by_vec_u8(&date, pe_start + 124+8);
            // 解析 data_directory
            let mut data_directory: Vec<structs::data_directory::DataDirectory> = Vec::new();
            for ie in 0..16 {
                let mut data_directory_item = structs::data_directory::DataDirectory {
                    virtual_address: 0,
                    size: 0,
                };
                let i = ie as usize;
                data_directory_item.virtual_address =
                    read_u32_by_vec_u8(&date, pe_start + 140+8 + i * 8);
                data_directory_item.size = read_u32_by_vec_u8(&date, pe_start + 144+8 + i * 8);
                data_directory.push(data_directory_item);
            }
            optional_header64.data_directory = data_directory;
            pe_header.optional_header = structs::optional_header::OptionalHeader::OptionalHeader64(optional_header64);
        }
        _ => {
            // 未知,可能没有
        }
    }

    Ok(pe_header)
}
fn read_u8_by_vec_u8(raw_date: &Vec<u8>, start: usize) -> u8 {
    return raw_date[start];
}

fn read_u16_by_vec_u8(raw_date: &Vec<u8>, start: usize) -> u16 {
    return u16::from_le_bytes([raw_date[start], raw_date[start + 1]]);
}
fn read_u32_by_vec_u8(raw_date: &Vec<u8>, start: usize) -> u32 {
    return u32::from_le_bytes([
        raw_date[start],
        raw_date[start + 1],
        raw_date[start + 2],
        raw_date[start + 3],
    ]);
}
fn read_u64_by_vec_u8(raw_date: &Vec<u8>, start: usize) -> u64 {
    return u64::from_le_bytes([
        raw_date[start],
        raw_date[start + 1],
        raw_date[start + 2],
        raw_date[start + 3],
        raw_date[start + 4],
        raw_date[start + 5],
        raw_date[start + 6],
        raw_date[start + 7],
    ]);
}
