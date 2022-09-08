//! This module defines the implementation of the Display trait for Input
//!
use super::Input;
use tabled::{object::Columns, Modify, Style, Table, Tabled, Width};

#[derive(Tabled)]
struct TableEntry {
    #[tabled(rename = "ID")]
    id: u16,
    #[tabled(rename = "File name")]
    filename: String,
    #[tabled(rename = "Offset (bytes)")]
    offset: u64,
    #[tabled(rename = "File size (bytes)")]
    filesize: u64,
    #[tabled(rename = "Header size (bytes)")]
    headersize: u64,
    #[tabled(rename = "Padding size (bytes)")]
    paddingsize: u64,
    #[tabled(rename = "Total size (bytes)")]
    total: u64,
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
                paddingsize: part.padding,
                total: part.header.offset() + part.padding,
                offset: part.offset,
            });
        }
        let table = Table::new(entries);
        let display = format!("{header}\n{table}");
        write!(f, "{}", display)
    }
}

impl Input {
    pub fn export_csv(&self) -> String {
        let mut entries = Vec::new();

        for part in &self.img_parts {
            entries.push(crate::img_header::display::TableEntry::from(&part.header));
        }

        let table = Table::new(entries).with(Style::blank().vertical(';'));
        format!("{table}")
    }

    pub fn full_table(&self) -> String {
        let mut entries = Vec::new();

        for part in &self.img_parts {
            entries.push(crate::img_header::display::TableEntry::from(&part.header));
        }

        let table = Table::new(entries)
            .with(
                Modify::new(Columns::single(0)).with(Width::wrap("Header size".len()).keep_words()),
            )
            .with(
                Modify::new(Columns::new(1..=3))
                    .with(Width::wrap("0x00".len() * 2 + 1).keep_words()),
            )
            .with(Modify::new(Columns::single(4)).with(Width::wrap("File size".len()).keep_words()))
            .with(
                Modify::new(Columns::new(10..=11))
                    .with(Width::wrap("Block size".len()).keep_words()),
            )
            .with(
                Modify::new(Columns::single(12))
                    .with(Width::wrap("Blank field".len()).keep_words()),
            )
            .with(
                Modify::new(Columns::single(13))
                    .with(Width::wrap("File checksum".len()).keep_words()),
            );

        format!("{table}")
    }
}
