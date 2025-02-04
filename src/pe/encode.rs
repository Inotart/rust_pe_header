use super::structs;
pub fn encode(data: &structs::PEHeader) -> Vec<u8> {
    let mut bytes = Vec::new();
    // signature
    bytes.extend_from_slice(&data.signature.to_le_bytes());
    // file_header
    {
        // machine
        bytes.extend_from_slice(&data.file_header.machine.to_le_bytes());
        // number_of_sections
        bytes.extend_from_slice(&data.file_header.number_of_sections.to_le_bytes());
        // time_date_stamp
        bytes.extend_from_slice(&data.file_header.time_date_stamp.to_le_bytes());
        // pointer_to_symbol_table
        bytes.extend_from_slice(&data.file_header.pointer_to_symbol_table.to_le_bytes());
        // number_of_symbols
        bytes.extend_from_slice(&data.file_header.number_of_symbols.to_le_bytes());
        // size_of_optional_header
        bytes.extend_from_slice(&data.file_header.size_of_optional_header.to_le_bytes());
        // characteristics
        bytes.extend_from_slice(&data.file_header.characteristics.to_le_bytes());
    }
    // optional_header
    if data.optional_header.is_some(){
        let optional_header = data.optional_header.as_ref().unwrap();
        // magic
        bytes.extend_from_slice(&optional_header.magic.to_le_bytes());
        // major_linker_version
        bytes.extend_from_slice(&optional_header.major_linker_version.to_le_bytes());
        // minor_linker_version
        bytes.extend_from_slice(&optional_header.minor_linker_version.to_le_bytes());
        // size_of_code
        bytes.extend_from_slice(&optional_header.size_of_code.to_le_bytes());
        // size_of_initialized_data
        bytes.extend_from_slice(&optional_header.size_of_initialized_data.to_le_bytes());
        // size_of_uninitialized_data
        bytes.extend_from_slice(&optional_header.size_of_uninitialized_data.to_le_bytes());
        // address_of_entry_point
        bytes.extend_from_slice(&optional_header.address_of_entry_point.to_le_bytes());
        // base_of_code
        bytes.extend_from_slice(&optional_header.base_of_code.to_le_bytes());
        // base_of_data
        bytes.extend_from_slice(&optional_header.base_of_data.to_le_bytes());
        // image_base
        bytes.extend_from_slice(&optional_header.image_base.to_le_bytes());
        // section_alignment
        bytes.extend_from_slice(&optional_header.section_alignment.to_le_bytes());
        // file_alignment
        bytes.extend_from_slice(&optional_header.file_alignment.to_le_bytes());
        // major_operating_system_version
        bytes.extend_from_slice(&optional_header.major_operating_system_version.to_le_bytes());
        // minor_operating_system_version
        bytes.extend_from_slice(&optional_header.minor_operating_system_version.to_le_bytes());
        // major_image_version
        bytes.extend_from_slice(&optional_header.major_image_version.to_le_bytes());
        // minor_image_version
        bytes.extend_from_slice(&optional_header.minor_image_version.to_le_bytes());
        // major_subsystem_version
        bytes.extend_from_slice(&optional_header.major_subsystem_version.to_le_bytes());
        // minor_subsystem_version
        bytes.extend_from_slice(&optional_header.minor_subsystem_version.to_le_bytes());
        // win32_version_value
        bytes.extend_from_slice(&optional_header.win32_version_value.to_le_bytes());
        // size_of_image
        bytes.extend_from_slice(&optional_header.size_of_image.to_le_bytes());
        // size_of_headers
        bytes.extend_from_slice(&optional_header.size_of_headers.to_le_bytes());
        // check_sum
        bytes.extend_from_slice(&optional_header.check_sum.to_le_bytes());
        // subsystem
        bytes.extend_from_slice(&optional_header.subsystem.to_le_bytes());
        // dll_characteristics
        bytes.extend_from_slice(&optional_header.dll_characteristics.to_le_bytes());
        // size_of_stack_reserve
        bytes.extend_from_slice(&optional_header.size_of_stack_reserve.to_le_bytes());
        // size_of_stack_commit
        bytes.extend_from_slice(&optional_header.size_of_stack_commit.to_le_bytes());
        // size_of_heap_reserve
        bytes.extend_from_slice(&optional_header.size_of_heap_reserve.to_le_bytes());
        // size_of_heap_commit
        bytes.extend_from_slice(&optional_header.size_of_heap_commit.to_le_bytes());
        // loader_flags
        bytes.extend_from_slice(&optional_header.loader_flags.to_le_bytes());
        // number_of_rva_and_sizes
        bytes.extend_from_slice(&optional_header.number_of_rva_and_sizes.to_le_bytes());
        for data_directory in &optional_header.data_directory {
            // virtual_address
            bytes.extend_from_slice(&data_directory.virtual_address.to_le_bytes());
            // size
            bytes.extend_from_slice(&data_directory.size.to_le_bytes());
            
            
        }

    } else if data.optional_header64.is_some(){
        let optional_header = data.optional_header64.as_ref().unwrap();
        // address_of_entry_point
        bytes.extend_from_slice(&optional_header.address_of_entry_point.to_le_bytes());
        // base_of_code
        bytes.extend_from_slice(&optional_header.base_of_code.to_le_bytes());
        // image_base
        bytes.extend_from_slice(&optional_header.image_base.to_le_bytes());
        // section_alignment
        bytes.extend_from_slice(&optional_header.section_alignment.to_le_bytes());
        // file_alignment
        bytes.extend_from_slice(&optional_header.file_alignment.to_le_bytes());
        // major_operating_system_version
        bytes.extend_from_slice(&optional_header.major_operating_system_version.to_le_bytes());
        // minor_operating_system_version
        bytes.extend_from_slice(&optional_header.minor_operating_system_version.to_le_bytes());
        // major_image_version
        bytes.extend_from_slice(&optional_header.major_image_version.to_le_bytes());
        // minor_image_version
        bytes.extend_from_slice(&optional_header.minor_image_version.to_le_bytes());
        // major_subsystem_version
        bytes.extend_from_slice(&optional_header.major_subsystem_version.to_le_bytes());
        // minor_subsystem_version
        bytes.extend_from_slice(&optional_header.minor_subsystem_version.to_le_bytes());
        // win32_version_value
        bytes.extend_from_slice(&optional_header.win32_version_value.to_le_bytes());
        // size_of_image
        bytes.extend_from_slice(&optional_header.size_of_image.to_le_bytes());
        // size_of_headers
        bytes.extend_from_slice(&optional_header.size_of_headers.to_le_bytes());
        // check_sum
        bytes.extend_from_slice(&optional_header.check_sum.to_le_bytes());
        // subsystem
        bytes.extend_from_slice(&optional_header.subsystem.to_le_bytes());
        // dll_characteristics
        bytes.extend_from_slice(&optional_header.dll_characteristics.to_le_bytes());
        // size_of_stack_reserve
        bytes.extend_from_slice(&optional_header.size_of_stack_reserve.to_le_bytes());
        // size_of_stack_commit
        bytes.extend_from_slice(&optional_header.size_of_stack_commit.to_le_bytes());
        // size_of_heap_reserve
        bytes.extend_from_slice(&optional_header.size_of_heap_reserve.to_le_bytes());
        // size_of_heap_commit
        bytes.extend_from_slice(&optional_header.size_of_heap_commit.to_le_bytes());
        // loader_flags
        bytes.extend_from_slice(&optional_header.loader_flags.to_le_bytes());
        // number_of_rva_and_sizes
        bytes.extend_from_slice(&optional_header.number_of_rva_and_sizes.to_le_bytes());
        for data_directory in &optional_header.data_directory {
            // virtual_address
            bytes.extend_from_slice(&data_directory.virtual_address.to_le_bytes());
            // size
            bytes.extend_from_slice(&data_directory.size.to_le_bytes());
        }
    }
    
    return bytes;
}