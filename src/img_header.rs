//! This module contains the layout of a packed image file header
//!
//! Adapted from [https://github.com/JoeyJiao/split_updata.pl].
//! According to further analysis, the file name is coded with 32 bytes, and not
//! 16 bytes followed by 16 null bytes (ERECOVERY_RAMDISK string need 17 bytes).
//! Following the same logic, the blocksize field is probably 4 bytes long and
//! not 2 bytes followed by 2 null bytes in a blank field.
//!
//! Each file starts with a magic number: 55AA 5AA5, followed by
//! - 4 bytes for Header Length
//! - 4 bytes for Unknown1
//! - 8 bytes for Hardware ID
//! - 4 bytes for File Sequence
//! - 4 bytes for File Size
//! - 16 bytes for File Date
//! - 16 bytes for File Time
//! - 32 bytes for File Type, containts the file name as utf-8
//! - 2 bytes for Header Checksum (Sum & 0xFFFF? or some kind of CRC16?)
//! - 4 bytes for BlockSize
//! - ($headerLength-98) bytes for file checksum
//! - data file length bytes for files.
//! - padding if necessary (so the total size of the chunk is a multiple of 4, i.e. 4-byte aligned)
//!
use crate::local_error::Error;
use crate::utils::remove_null_bytes;

pub mod display;

#[derive(Default, Clone)]
pub struct ImgHeader {
    pub header_len: [u8; 4],
    pub unknown_field: [u8; 4],
    pub hardware_id: [u8; 8],
    pub file_sequence: [u8; 4],
    pub file_size: [u8; 4],
    pub file_date: [u8; 16],
    pub file_time: [u8; 16],
    pub file_type: [u8; 32],
    pub header_checksum: [u8; 2],
    pub blocksize: [u8; 4],
    pub file_checksum_size: u32, // ($header_len - 98) should fit in u32 as header_len is u32
}

/// Magic number showing the presence of an img chunk
const MAGIC_NUMBER: [u8; 4] = [0x55, 0xAA, 0x5A, 0xA5];
pub const MIN_DATA_LEN: usize = 102; // 98 bytes for the header + 4 bytes for the size of the data/file_checksum
pub const MIN_HEADER_LEN: u32 = 98;
pub const FILE_CHECKSUM_OFFSET: u64 = 98;

impl std::convert::TryFrom<&[u8]> for ImgHeader {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < MIN_HEADER_LEN as usize {
            return Err(String::from("Unable to parse: not enough data provided"));
        }
        if data[0..=3] != MAGIC_NUMBER {
            return Err(String::from(
                "Unable to parse: data doesn't start with magic number",
            ));
        }
        let mut img = ImgHeader {
            header_len: data[4..=7].try_into().map_err(|e| format!("{}", e))?,
            unknown_field: data[8..=11].try_into().map_err(|e| format!("{}", e))?,
            hardware_id: data[12..=19].try_into().map_err(|e| format!("{}", e))?,
            file_sequence: data[20..=23].try_into().map_err(|e| format!("{}", e))?,
            file_size: data[24..=27].try_into().map_err(|e| format!("{}", e))?,
            file_date: data[28..=43].try_into().map_err(|e| format!("{}", e))?,
            file_time: data[44..=59].try_into().map_err(|e| format!("{}", e))?,
            file_type: data[60..=91].try_into().map_err(|e| format!("{}", e))?,
            header_checksum: data[92..=93].try_into().map_err(|e| format!("{}", e))?,
            blocksize: data[94..=97].try_into().map_err(|e| format!("{}", e))?,
            ..Self::default()
        };
        let header_len = u32::from_le_bytes(img.header_len);
        if header_len < MIN_HEADER_LEN {
            return Err(format!(
                "Unable to parse: header is too small ({} bytes)",
                header_len
            ));
        }
        img.file_checksum_size = header_len - MIN_HEADER_LEN;
        Ok(img)
    }
}

impl ImgHeader {
    pub fn filename(&self) -> Result<String, Error> {
        String::from_utf8(remove_null_bytes(self.file_type.as_slice())).map_err(Error::from)
    }

    pub fn filename_lossy(&self) -> String {
        String::from_utf8_lossy(remove_null_bytes(self.file_type.as_slice()).as_slice())
            .into_owned()
    }

    pub fn filesize(&self) -> u64 {
        // return a u64 as it is the same type as SeekFrom argument/offset
        u32::from_le_bytes(self.file_size) as u64
    }

    /// Return the full size in bytes of the img data including header and padding.
    pub fn offset(&self) -> u64 {
        self.filesize() + self.headersize()
    }

    /// Returns the length of the header.
    pub fn headersize(&self) -> u64 {
        u32::from_le_bytes(self.header_len) as u64
    }

    /// Returns the blocksize
    pub fn blocksize(&self) -> u64 {
        u32::from_le_bytes(self.blocksize) as u64
    }

    /// Returns the size of the file checksum field
    pub fn filechecksumsize(&self) -> usize {
        self.file_checksum_size as usize
    }
}

#[cfg(test)]
mod tests {
    mod try_from {
        use crate::img_header::ImgHeader;
        use std::convert::TryFrom;

        #[test]
        fn empty_data() {
            let result = ImgHeader::try_from(vec![].as_slice());

            assert!(result.is_err());
        }

        #[test]
        fn short_data() {
            let result = ImgHeader::try_from(
                vec![0x55, 0xAA, 0x5A, 0xA5, 0x55, 0xAA, 0x5A, 0xA5].as_slice(),
            );

            assert!(result.is_err());
        }

        #[test]
        fn no_magic_number() {
            let data = vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
                0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03,
                0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01,
                0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
                0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03,
                0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01,
                0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            ];

            let result = ImgHeader::try_from(data.as_slice());

            assert!(result.is_err());
        }
    }
}
