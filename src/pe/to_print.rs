use chrono::DateTime;


use super::{machine::get_machine_type, structs::{self, word_to_string,word_to_string_32}, subsystem::get_subsystem_type};

pub fn print(header: structs::PEHeader) -> Result<String, Box<dyn std::error::Error>> {
    let mut words = String::new();
    words.push_str("PEHeader { \n");
    words.push_str(format!("\tsignature: {}\n", word_to_string_32(header.signature)?).as_str());
    // signature
    {
        words.push_str("\tFileHeader: { \n");
        words.push_str(format!("\t\tMachine: {}\n", get_machine_type(header.file_header.machine)).as_str());
        words.push_str(format!("\t\tNumberOfSections: {}\n", header.file_header.number_of_sections).as_str());
        words.push_str(format!("\t\tTimeDateStamp: {:?}\n", DateTime::from_timestamp(header.file_header.time_date_stamp.into(), 0).expect("invalid timestamp")).as_str());
        words.push_str(format!("\t\tPointerToSymbolTable: {}\n", header.file_header.pointer_to_symbol_table).as_str());
        words.push_str(format!("\t\tNumberOfSymbols: {}\n", header.file_header.number_of_symbols).as_str());
        words.push_str(format!("\t\tSizeOfOptionalHeader: {}\n", header.file_header.size_of_optional_header).as_str());
        words.push_str(format!("\t\tCharacteristics: {}\n", header.file_header.characteristics).as_str());
        words.push_str("\t},\n");
    }
    if header.optional_header.is_some() {
        words.push_str("\tOptionalHeader: { \n");
        let optional_header = header.optional_header.unwrap();
        words.push_str(format!("\t\tMagic: {}\n", optional_header.magic).as_str());
        words.push_str(format!("\t\tMajorLinkerVersion: {}\n", optional_header.major_linker_version).as_str());
        words.push_str(format!("\t\tMinorLinkerVersion: {}\n", optional_header.minor_linker_version).as_str());
        words.push_str(format!("\t\tSizeOfCode: {}\n", optional_header.size_of_code).as_str());
        words.push_str(format!("\t\tSizeOfInitializedData: {}\n", optional_header.size_of_initialized_data).as_str());
        words.push_str(format!("\t\tSizeOfUninitializedData: {}\n", optional_header.size_of_uninitialized_data).as_str());
        words.push_str(format!("\t\tAddressOfEntryPoint: {}\n", optional_header.address_of_entry_point).as_str());
        words.push_str(format!("\t\tBaseOfCode: {}\n", optional_header.base_of_code).as_str());
        words.push_str(format!("\t\tBaseOfData: {}\n", optional_header.base_of_data).as_str());
        words.push_str(format!("\t\tImageBase: {}\n", optional_header.image_base).as_str());
        words.push_str(format!("\t\tSectionAlignment: {}\n", optional_header.section_alignment).as_str());
        words.push_str(format!("\t\tFileAlignment: {}\n", optional_header.file_alignment).as_str());
        words.push_str(format!("\t\tMajorOperatingSystemVersion: {}\n", optional_header.major_operating_system_version).as_str());
        words.push_str(format!("\t\tMinorOperatingSystemVersion: {}\n", optional_header.minor_operating_system_version).as_str());
        words.push_str(format!("\t\tMajorImageVersion: {}\n", optional_header.major_image_version).as_str());
        words.push_str(format!("\t\tMinorImageVersion: {}\n", optional_header.minor_image_version).as_str());
        words.push_str(format!("\t\tMajorSubsystemVersion: {}\n", optional_header.major_subsystem_version).as_str());
        words.push_str(format!("\t\tMinorSubsystemVersion: {}\n", optional_header.minor_subsystem_version).as_str());
        words.push_str(format!("\t\tWin32VersionValue: {}\n", optional_header.win32_version_value).as_str());
        words.push_str(format!("\t\tSizeOfImage: {}\n", optional_header.size_of_image).as_str());
        words.push_str(format!("\t\tSizeOfHeaders: {}\n", optional_header.size_of_headers).as_str());
        words.push_str(format!("\t\tCheckSum: {}\n", optional_header.check_sum).as_str());
        words.push_str(format!("\t\tSubsystem: {}\n", optional_header.subsystem).as_str());
        words.push_str(format!("\t\tDllCharacteristics: {}\n", optional_header.dll_characteristics).as_str());
        words.push_str(format!("\t\tSizeOfStackReserve: {}\n", optional_header.size_of_stack_reserve).as_str());
        words.push_str(format!("\t\tSizeOfStackCommit: {}\n", optional_header.size_of_stack_commit).as_str());
        words.push_str(format!("\t\tSizeOfHeapReserve: {}\n", optional_header.size_of_heap_reserve).as_str());
        words.push_str(format!("\t\tSizeOfHeapCommit: {}\n", optional_header.size_of_heap_commit).as_str());
        words.push_str(format!("\t\tLoaderFlags: {}\n", optional_header.loader_flags).as_str());
        words.push_str(format!("\t\tNumberOfRvaAndSizes: {}\n", optional_header.number_of_rva_and_sizes).as_str());
        words.push_str(&format!("\t\tdata_directory: {:?}\n", optional_header.data_directory));
        words.push_str("\n\t}");

        
    }
    if header.optional_header64.is_some() {
        let optional_header = header.optional_header64.unwrap();
        words.push_str("\tOptionalHeader64: {");
        words.push_str(format!("\t\tmagic: {}\n", optional_header.magic).as_str());
        words.push_str(format!("\t\tmajor_linker_version: {}\n", optional_header.major_linker_version).as_str());
        words.push_str(format!("\t\tminor_linker_version: {}\n", optional_header.minor_linker_version).as_str());
        words.push_str(format!("\t\tsize_of_code: {}\n", optional_header.size_of_code).as_str());
        words.push_str(format!("\t\tsize_of_initialized_data: {}\n", optional_header.size_of_initialized_data).as_str());
        words.push_str(format!("\t\tsize_of_uninitialized_data: {}\n", optional_header.size_of_uninitialized_data).as_str());
        words.push_str(format!("\t\taddress_of_entry_point: {}\n", optional_header.address_of_entry_point).as_str());
        words.push_str(format!("\t\tbase_of_code: {}\n", optional_header.base_of_code).as_str());
        words.push_str(format!("\t\timage_base: {}\n", optional_header.image_base).as_str());
        words.push_str(format!("\t\tsection_alignment: {}\n", optional_header.section_alignment).as_str());
        words.push_str(format!("\t\tfile_alignment: {}\n", optional_header.file_alignment).as_str());
        words.push_str(format!("\t\tmajor_operating_system_version: {}\n", optional_header.major_operating_system_version).as_str());
        words.push_str(format!("\t\tminor_operating_system_version: {}\n", optional_header.minor_operating_system_version).as_str());
        words.push_str(format!("\t\tmajor_image_version: {}\n", optional_header.major_image_version).as_str());
        words.push_str(format!("\t\tminor_image_version: {}\n", optional_header.minor_image_version).as_str());
        words.push_str(format!("\t\tmajor_subsystem_version: {}\n", optional_header.major_subsystem_version).as_str());
        words.push_str(format!("\t\tminor_subsystem_version: {}\n", optional_header.minor_subsystem_version).as_str());
        words.push_str(format!("\t\twin32_version_value: {}\n", optional_header.win32_version_value).as_str());
        words.push_str(format!("\t\tsize_of_image: {}\n", optional_header.size_of_image).as_str());
        words.push_str(format!("\t\tsize_of_headers: {}\n", optional_header.size_of_headers).as_str());
        words.push_str(format!("\t\tchecksum: {}\n", optional_header.check_sum).as_str());
        words.push_str(format!("\t\tsubsystem: {}\n", get_subsystem_type(optional_header.subsystem)).as_str());
        words.push_str(format!("\t\tdll_characteristics: {}\n", optional_header.dll_characteristics).as_str());
        words.push_str(format!("\t\tsize_of_stack_reserve: {}\n", optional_header.size_of_stack_reserve).as_str());
        words.push_str(format!("\t\tsize_of_stack_commit: {}\n", optional_header.size_of_stack_commit).as_str());
        words.push_str(format!("\t\tsize_of_heap_reserve: {}\n", optional_header.size_of_heap_reserve).as_str());
        words.push_str(format!("\t\tsize_of_heap_commit: {}\n", optional_header.size_of_heap_commit).as_str());
        words.push_str(format!("\t\tloader_flags: {}\n", optional_header.loader_flags).as_str());
        words.push_str(format!("\t\tnumber_of_rva_and_sizes: {}\n", optional_header.number_of_rva_and_sizes).as_str());
        words.push_str(&format!("\t\tdata_directory: {:?}\n", optional_header.data_directory));
        words.push_str("\n\t}");
        
    }
    words.push_str("}");

    Ok(words)    
}
