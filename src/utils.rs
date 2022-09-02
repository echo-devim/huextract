//! This module contains utility functions
//!

pub fn remove_null_bytes(buffer: [u8; 16]) -> Vec<u8> {
    let mut pos = 15;
    while buffer[pos] == 0x00 && pos != 0 {
        pos -= 1;
    }
    Vec::from(&buffer[..=pos])
}
