//! This module describes a packed img file
//!

use crate::img_header::ImgHeader;

pub struct Img {
    pub header: ImgHeader,
    pub offset: u64,
}

impl Img {
    pub fn new(header: ImgHeader, offset: u64) -> Self {
        Self { header, offset }
    }
}

impl std::fmt::Display for Img {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Header:\n{}\nOffset: {}", self.header, self.offset)
    }
}
