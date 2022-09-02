//! This module defines the implementation of the Display trait for Input
//!
use super::Input;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct TableEntry {
    #[tabled(rename = "ID")]
    id: u16,
    #[tabled(rename = "File name")]
    filename: String,
    #[tabled(rename = "File size (bytes)")]
    filesize: u64,
    #[tabled(rename = "Header size (bytes)")]
    headersize: u64,
    #[tabled(rename = "Total size (bytes)")]
    total: u64,
    #[tabled(rename = "Offset (bytes)")]
    offset: u64,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = format!("Filename: {}, size: {} bytes\n", self.filename, self.size);
        let mut entries = Vec::new();
        for (i, part) in self.img_parts.iter().enumerate() {
            entries.push(TableEntry {
                id: (i + 1) as u16,
                filename: part.header.filename_lossy(),
                filesize: part.header.filesize(),
                headersize: part.header.headersize(),
                total: part.header.filesize() + part.header.headersize(),
                offset: part.offset,
            });
        }
        let table = Table::new(entries);
        let display = format!("{header}\n{table}");
        write!(f, "{}", display)
    }
}
