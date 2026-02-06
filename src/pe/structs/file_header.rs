use std::fmt;

use chrono::DateTime;

use crate::pe::structs::{DWord, Word};

#[repr(C)]
#[derive(Debug)]
pub struct FileHeader {
    pub machine: Word,                  // 机器类型
    pub number_of_sections: Word,       // 文件中的节区数量
    pub time_date_stamp: DWord,         // 文件创建时间
    pub pointer_to_symbol_table: DWord, // 指向符号表的指针
    pub number_of_symbols: DWord,       // 符号表中的符号数量
    pub size_of_optional_header: Word,  // 可选头的大小
    pub characteristics: Word,          // 文件属性
}
impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ts = self.time_date_stamp.into();
        let time_date_stamp_str = match DateTime::from_timestamp(ts, 0) {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => format!("0x{:X} (invalid)", ts),
        };
        f.debug_struct("FileHeader")
            .field(
                "machine",
                &format_args!("{:?}", crate::pe::machine::get_machine_type(self.machine)),
            )
            .field("number_of_sections", &self.number_of_sections)
            .field("time_date_stamp", &time_date_stamp_str)
            .field(
                "pointer_to_symbol_table",
                &format_args!("0x{:X}", self.pointer_to_symbol_table),
            )
            .field("number_of_symbols", &self.number_of_symbols)
            .field("size_of_optional_header", &self.size_of_optional_header)
            .field(
                "characteristics",
                &format_args!("{:?}", self.characteristics),
            )
            .finish()
    }
}
