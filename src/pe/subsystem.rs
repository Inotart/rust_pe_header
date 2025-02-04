pub fn get_subsystem_type(machine: u16) -> &'static str {
    match machine {
        0 => "UNKNOWN",
        1 => "NATIVE",
        2 => "WINDOWS_GUI",
        3 => "WINDOWS_CUI",
        5 => "OS2_CUI",
        7 => "POSIX_CUI",
        8 => "NATIVE_WINDOWS",
        9 => "WINDOWS_CE_GUI",
        10 => "EFI_APPLICATION",
        11 => "EFI_BOOT_ SERVICE_DRIVER",
        12 => "EFI_RUNTIME_ DRIVER",
        13 => "EFI_ROM",
        14 => "XBOX",
        16 => "WINDOWS_BOOT_APPLICATION",
        _ => "UNKNOWN",
    }
}
