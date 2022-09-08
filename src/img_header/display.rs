//! This module defines the implementation of the Display trait for ImgHeader
//!
use super::ImgHeader;
use tabled::{Table, Tabled};

use crate::tabled_types::{ArrayValue, HexValue};
use crate::utils::remove_null_bytes;

#[derive(Tabled)]
pub struct TableEntry {
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
    #[tabled(rename = "Header checksum")]
    header_checksum: HexValue,
    #[tabled(rename = "Block size (raw)")]
    blocksize_raw: ArrayValue,
    #[tabled(rename = "Block size (bytes)")]
    blocksize: u64,
    #[tabled(rename = "File checksum size (bytes)")]
    file_checksum_size: u32,
}

impl From<&ImgHeader> for TableEntry {
    fn from(header: &ImgHeader) -> Self {
        TableEntry {
            header_len: u32::from_le_bytes(header.header_len),
            unknown_field: ArrayValue::from(header.unknown_field.as_slice()),
            hardware_id: ArrayValue::from(header.hardware_id.as_slice()),
            file_sequence: ArrayValue::from(header.file_sequence.as_slice()),
            file_size: u32::from_le_bytes(header.file_size),
            file_date: ArrayValue::from(header.file_date.as_slice()),
            file_time: ArrayValue::from(header.file_time.as_slice()),
            file_type: header.filename_lossy(),
            header_checksum: HexValue::from(header.header_checksum.as_slice()),
            blocksize_raw: ArrayValue::from(header.blocksize.as_slice()),
            blocksize: header.blocksize(),
            file_checksum_size: header.file_checksum_size,
        }
    }
}

impl std::fmt::Display for ImgHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entries = TableEntry::from(self);
        let table = Table::new(vec![entries]);

        write!(f, "{}", table)
    }
}

#[derive(Tabled)]
pub struct CsvEntry {
    #[tabled(rename = "Header size (bytes)")]
    header_len: u32,
    #[tabled(rename = "Unknown field")]
    unknown_field: u32,
    #[tabled(rename = "Hardware ID")]
    hardware_id: u64,
    #[tabled(rename = "File sequence")]
    file_sequence: u32,
    #[tabled(rename = "File size (bytes)")]
    file_size: u32,
    #[tabled(rename = "File date")]
    file_date: String,
    #[tabled(rename = "File time")]
    file_time: String,
    #[tabled(rename = "File name")]
    file_type: String,
    #[tabled(rename = "Header checksum")]
    header_checksum: u16,
    #[tabled(rename = "Block size (bytes)")]
    blocksize: u32,
    #[tabled(rename = "File checksum size (bytes)")]
    file_checksum_size: u32,
}

impl From<&ImgHeader> for CsvEntry {
    fn from(header: &ImgHeader) -> Self {
        Self {
            header_len: u32::from_le_bytes(header.header_len),
            unknown_field: u32::from_le_bytes(header.unknown_field),
            hardware_id: u64::from_le_bytes(header.hardware_id),
            file_sequence: u32::from_le_bytes(header.file_sequence),
            file_size: u32::from_le_bytes(header.file_size),
            file_date: String::from_utf8_lossy(&remove_null_bytes(header.file_date.as_slice()))
                .to_string(),
            file_time: String::from_utf8_lossy(&remove_null_bytes(header.file_time.as_slice()))
                .to_string(),
            file_type: header.filename_lossy(),
            header_checksum: u16::from_le_bytes(header.header_checksum),
            blocksize: u32::from_le_bytes(header.blocksize),
            file_checksum_size: header.file_checksum_size,
        }
    }
}
