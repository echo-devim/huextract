//! This module contains utility functions
//!

pub fn remove_null_bytes(buffer: &[u8]) -> Vec<u8> {
    if buffer.is_empty() {
        Vec::new()
    } else {
        let mut pos = buffer.len() - 1;
        while buffer[pos] == 0x00 && pos != 0 {
            pos -= 1;
        }
        Vec::from(&buffer[..=pos])
    }
}
