//! This module allows to calculate a CRC checksum as in the Huawei firmwares
//!
//! This is based on the implementation in C# of https://github.com/worstenbrood/HuaweiUpdateLibrary
//! (see
//! https://github.com/worstenbrood/HuaweiUpdateLibrary/blob/master/HuaweiUpdateLibrary/Algorithms/UpdateCrc16.cs)
//!
use std::io::{BufRead, Seek, SeekFrom};

trait SeekRead: Seek + BufRead {}
impl<T: Seek + BufRead> SeekRead for T {}

pub struct Crc {
    table: [u16; 256],
    hash_value: u16,
    blocksize: usize,
}

const INITIAL_SUM: u16 = 0xFFFF;
const POLYNOMIAL: u16 = 0x8408;
const XOR_VALUE: u16 = 0xFFFF;

impl Crc {
    pub fn new(blocksize: usize) -> Self {
        let mut new = Crc {
            table: [0; 256],
            hash_value: 0,
            blocksize,
        };
        for i in 0..new.table.len() {
            let mut value: u16 = 0;
            let mut temp: u16 = i as u16;
            for _j in 0..8 {
                if ((value ^ temp) & 0x0001) != 0 {
                    value = (value >> 1) ^ POLYNOMIAL;
                } else {
                    value >>= 1;
                }
                temp >>= 1;
            }
            new.table[i] = value;
        }
        new.hash_value = INITIAL_SUM;
        new
    }

    fn hash_core(&mut self, array: &[u8], start: usize, count: usize) {
        let mut sum = self.hash_value;
        let mut i = start;
        let mut size = (count - start) * 8;

        while size >= 8 {
            let v = array[i];
            sum = self.table[(v ^ sum as u8) as usize] ^ (sum >> 8);
            size -= 8;
            i += 1;
        }

        if size != 0 {
            let mut n = 0; // Equivalent to original code: n = array[i] << 8;
            loop {
                if size == 0 {
                    break;
                }
                size -= 1;
                let flag = ((sum as u8 ^ n) & 1) == 0;
                sum >>= 1;
                if flag {
                    sum ^= POLYNOMIAL;
                }
                n >>= 1;
            }
        }

        self.hash_value = sum;
    }

    fn hash_final(&mut self) -> Vec<u8> {
        let result = self.hash_value ^ XOR_VALUE;

        // Reinit
        self.hash_value = INITIAL_SUM;
        result.to_le_bytes().to_vec()
    }

    pub fn compute_checksum(&mut self, data: &[u8]) -> Vec<u8> {
        let mut checksum = Vec::new();
        let size = data.len();
        let mut offset = 0;
        while offset < size {
            let remaining = size - offset;
            let count = std::cmp::min(remaining, self.blocksize);
            self.hash_core(data, offset, count);
            checksum.append(&mut self.hash_final());
            offset += count;
        }
        checksum
    }

    pub fn compute_file_checksum(
        &mut self,
        data: &mut dyn SeekRead,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut checksum = Vec::new();
        let mut bytes_read = 0;
        let size = data.seek(SeekFrom::End(0))? as usize;
        data.seek(SeekFrom::Start(0))?;
        let mut tmp = vec![0; self.blocksize];
        while bytes_read < size {
            let remaining = size - bytes_read;
            tmp.truncate(std::cmp::min(remaining, self.blocksize));
            let count = data.read(&mut tmp)?;
            self.hash_core(&tmp, 0, count);
            checksum.append(&mut self.hash_final());
            bytes_read += count;
        }

        Ok(checksum)
    }
}
