//! This module describes a packed img file
//!

use crate::img_header::ImgHeader;

#[derive(Clone)]
pub struct Img {
    pub header: ImgHeader,
    pub offset: u64,
    pub padding: u64,
}

impl Img {
    pub fn new(header: ImgHeader, offset: u64, padding: u64) -> Self {
        Self {
            header,
            offset,
            padding,
        }
    }
}

impl std::fmt::Display for Img {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Header:\n{}\nOffset: {}\nPadding: {}",
            self.header, self.offset, self.padding
        )
    }
}
