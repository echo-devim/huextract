/** This module contains the layout of a packed image file
 *
 */

/// Describes the layout of an img file
///
/// Each file starts with a magic number: 55AA 5AA5, followed by
/// - 4 bytes for Header Length
/// - 4 bytes for Unknown1
/// - 8 bytes for Hardware ID
/// - 4 bytes for File Sequence
/// - 4 bytes for File Size
/// - 16 bytes for File Date
/// - 16 bytes for File Time
/// - 16 bytes for File Type
/// - 16 bytes for Blank1
/// - 2 bytes for Header Checksum
/// - 2 bytes for BlockSize
/// - 2 bytes for Blank2
/// - ($headerLength-98) bytes for file checksum
/// - data file length bytes for files.
/// - padding ifnecessary
#[derive(Default)]
struct Img {
    header_len: u32,
    unknown_field: u32,
    hardware_id: u64,
    file_sequence: u32,
    file_size: u32,
    file_date: u128,
    file_time: u128,
    file_type: u128,
    blank_field1: u128,
    header_checksum: u16,
    blocksize: u16,
    blank_field2: u16,
    file_checksum: u32, // ($header_len - 98) should fit in u32 as header_len is u32
    data: Vec<u8>,
}

/// Magic number showing the presence of an img chunk
const MAGIC_NUMBER: [u8; 4] = [0x55, 0xAA, 0x5A, 0xA5];
const MIN_DATA_LEN: usize = 102; // 98 bytes for the header + 4 bytes for the size of the data/file_checksum
const MIN_HEADER_LEN: u32 = 98;

// impl std::fmt::Display for Img {
//     unimplemented!()
// }

impl std::convert::TryFrom<&[u8]> for Img {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < MIN_DATA_LEN {
            return Err(String::from("Unable to parse: not enough data provided"));
        }
        if data[0..3] != MAGIC_NUMBER {
            return Err(String::from(
                "Unable to parse: data doesn't start with magic number",
            ));
        }
        let mut img = Img {
            header_len: u32::from_le_bytes(data[4..7].try_into().map_err(|e| format!("{}", e))?),
            unknown_field: u32::from_le_bytes(
                data[8..11].try_into().map_err(|e| format!("{}", e))?,
            ),
            hardware_id: u64::from_le_bytes(data[12..19].try_into().map_err(|e| format!("{}", e))?),
            file_sequence: u32::from_le_bytes(
                data[20..23].try_into().map_err(|e| format!("{}", e))?,
            ),
            file_size: u32::from_le_bytes(data[24..27].try_into().map_err(|e| format!("{}", e))?),
            file_date: u128::from_le_bytes(data[28..43].try_into().map_err(|e| format!("{}", e))?),
            file_time: u128::from_le_bytes(data[44..59].try_into().map_err(|e| format!("{}", e))?),
            file_type: u128::from_le_bytes(data[60..75].try_into().map_err(|e| format!("{}", e))?),
            blank_field1: u128::from_le_bytes(
                data[76..91].try_into().map_err(|e| format!("{}", e))?,
            ),
            header_checksum: u16::from_le_bytes(
                data[92..93].try_into().map_err(|e| format!("{}", e))?,
            ),
            blocksize: u16::from_le_bytes(data[94..95].try_into().map_err(|e| format!("{}", e))?),
            blank_field2: u16::from_le_bytes(
                data[96..97].try_into().map_err(|e| format!("{}", e))?,
            ),
            ..Self::default()
        };
        if img.header_len < MIN_HEADER_LEN {
            return Err(format!(
                "Unable to parse: header is too small ({} bytes)",
                img.header_len
            ));
        }
        img.file_checksum = img.header_len - MIN_HEADER_LEN;
        img.data = data[MIN_DATA_LEN..].to_vec();
        while let Some(el) = img.data.pop() {
            if el != 0 {
                img.data.push(el);
                break;
            }
        }
        Ok(img)
    }
}

#[cfg(test)]
mod tests {
    mod try_from {
        use crate::img::Img;
        use std::convert::TryFrom;

        #[test]
        fn empty_data() {
            let result = Img::try_from(vec![].as_slice());

            assert!(result.is_err());
        }

        #[test]
        fn short_data() {
            let result =
                Img::try_from(vec![0x55, 0xAA, 0x5A, 0xA5, 0x55, 0xAA, 0x5A, 0xA5].as_slice());

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

            let result = Img::try_from(data.as_slice());

            assert!(result.is_err());
        }
    }
}
