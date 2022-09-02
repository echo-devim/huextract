//! This module defines the implementation of the Display trait for ImgHeader
//!
use super::ImgHeader;
use tabled::{Table, Tabled};

use crate::tabled_types::{ArrayValue, HexValue};

#[derive(Tabled)]
struct TableEntry {
    #[tabled(rename = "Header size (bytes)")]
    header_len: u32,
    #[tabled(rename = "Unknown field")]
    unknown_field: ArrayValue,
    #[tabled(rename = "Hardware ID")]
    hardware_id: ArrayValue,
    #[tabled(rename = "File sequence")]
    file_sequence: ArrayValue,
    #[tabled(rename = "File size (bytes)")]
    file_size: u32,
    #[tabled(rename = "File date")]
    file_date: ArrayValue,
    #[tabled(rename = "File time")]
    file_time: ArrayValue,
    #[tabled(rename = "File name")]
    file_type: String,
    #[tabled(rename = "Blank field (1)")]
    blank_field1: ArrayValue,
    #[tabled(rename = "Header checksum")]
    header_checksum: HexValue,
    #[tabled(rename = "Block size (raw)")]
    blocksize_raw: ArrayValue,
    #[tabled(rename = "Block size (bytes)")]
    blocksize: u64,
    #[tabled(rename = "Blank field (2)")]
    blank_field2: ArrayValue,
    #[tabled(rename = "File checksum size (bytes)")]
    file_checksum_size: u32,
}

impl std::fmt::Display for ImgHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entries = TableEntry {
            header_len: u32::from_le_bytes(self.header_len),
            unknown_field: ArrayValue::from(self.unknown_field.as_slice()),
            hardware_id: ArrayValue::from(self.hardware_id.as_slice()),
            file_sequence: ArrayValue::from(self.file_sequence.as_slice()),
            file_size: u32::from_le_bytes(self.file_size),
            file_date: ArrayValue::from(self.file_date.as_slice()),
            file_time: ArrayValue::from(self.file_time.as_slice()),
            file_type: self.filename_lossy(),
            blank_field1: ArrayValue::from(self.blank_field1.as_slice()),
            header_checksum: HexValue::from(self.header_checksum.as_slice()),
            blocksize_raw: ArrayValue::from(self.blocksize.as_slice()),
            blocksize: self.blocksize(),
            blank_field2: ArrayValue::from(self.blank_field2.as_slice()),
            file_checksum_size: self.file_checksum_size,
        };
        let table = Table::new(vec![entries]);

        write!(f, "{}", table)
    }
}
