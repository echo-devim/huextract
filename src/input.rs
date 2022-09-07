//! This module defines the representation of the input file based on its expected layout:
//!
//! ```
//! |---------------------------------------------------------|
//! | 0x00 * 92 | Img header + data | ... | Img header + data |
//! |---------------------------------------------------------|
//! ```
//!
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::Path;

use crate::crc::Crc;
use crate::img::Img;
use crate::img_header;
use crate::img_header::{ImgHeader, MIN_DATA_LEN, MIN_HEADER_LEN};
use crate::local_error::Error;

mod display;

pub struct Input {
    /// Buffer containing the input data
    data: BufReader<File>,
    /// Vector containing the different headers and their offset
    img_parts: Vec<Img>,
    /// Size of the input file
    pub size: u64,
    /// File name we got the data from
    filename: String,
    // /// File containing the input data
    // file: File,
    // /// Pointer to the data
    // data: &'a [u8],
}

impl std::convert::TryFrom<&Path> for Input {
    type Error = String;
    /// Create an instance of Input from a Path
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(path).map_err(|e| format!("{e}"))?;
        let size = file.metadata().map_err(|e| format!("{e}"))?.len();
        let data = BufReader::new(file);
        Ok(Input {
            data,
            size,
            img_parts: Vec::new(),
            filename: format!("{}", path.display()),
        })
    }
}

impl Input {
    /// Validate the input data: returns true if the data is valid, false elsewise.
    ///
    /// The input data must start with 92 bytes containing 0x00.
    pub fn validate(&mut self) -> Result<(), Error> {
        self.data.rewind()?;
        let mut head_content = [0; 92];
        self.data.read_exact(&mut head_content)?;
        if head_content != [0; 92] {
            Err(Error::from("File doesn't contain a valid data header"))
        } else {
            Ok(())
        }
    }

    /// Get the headers of the packed img files.
    ///
    /// Returns a Vec<Img>.
    pub fn parse(&mut self) -> Result<(), Error> {
        let end = self.data.seek(SeekFrom::End(0))?;
        self.data.seek(SeekFrom::Start(92))?;
        let mut offset = self.data.stream_position()?;
        let mut padding = 0;
        while (offset + MIN_DATA_LEN as u64) < end {
            let mut buf = [0; MIN_DATA_LEN as usize];
            self.data.read_exact(&mut buf)?;
            match ImgHeader::try_from(buf.as_slice()) {
                Ok(header) => {
                    self.img_parts
                        .push(Img::new(header.to_owned(), offset, padding));
                    offset += header.offset();
                    padding = 0;
                }
                Err(_) => {
                    padding += 1;
                    offset += 1;
                }
            }
            self.data.seek(SeekFrom::Start(offset))?;
        }
        let mut remaining: i128 = (end - offset) as i128;
        while remaining > 0 {
            let mut byte = [0; 1];
            self.data.read_exact(&mut byte)?;
            offset += 1;
            remaining -= 1;
            self.data.seek(SeekFrom::Start(offset))?;
        }
        Ok(())
    }

    /// Extract the content of the img files to disk
    pub fn extract(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    /// Extract the checksum file to the disk
    pub fn extract_checksum(&mut self) -> Result<(), Error> {
        for part in self.img_parts.clone() {
            let filename = format!("{}.sum", part.header.filename()?);
            let offset = part.offset + MIN_HEADER_LEN as u64;
            let size = (part.header.headersize() - MIN_HEADER_LEN as u64) as usize;
            self.write_to_disk(filename.as_str(), offset, size)?;
        }
        Ok(())
    }

    /// Extract the content of the img files to disk
    pub fn extract_img(&mut self) -> Result<(), Error> {
        for part in self.img_parts.clone() {
            let filename = format!("{}.img", part.header.filename()?);
            let offset = part.offset + part.header.headersize();
            let size = part.header.filesize() as usize;
            print!("Extracting {filename}... ");
            self.write_to_disk(&filename, offset, size)?;
            print!("Done");

            // Verify file checksum
            let mut checksum = Vec::new();
            self.write_to(
                &mut checksum,
                part.offset + img_header::FILE_CHECKSUM_OFFSET,
                part.header.filechecksumsize(),
            )?;
            let mut img = BufReader::new(File::open(&filename)?);
            print!("... Verifying checksum... ");
            if checksum
                == Crc::new(part.header.blocksize() as usize).compute_file_checksum(&mut img)?
            {
                println!("OK");
            } else {
                println!("Error");
            }
        }
        Ok(())
    }

    /// Helper function: writes given data to disk
    fn write_to_disk(&mut self, filename: &str, offset: u64, size: usize) -> Result<(), Error> {
        if File::open(&filename).is_ok() {
            return Err(Error::new(format!("File {} already exists", filename)));
        }

        let mut output_file = File::create(filename)?;
        self.write_to(&mut output_file, offset, size)?;
        Ok(())
    }

    /// Helper function: writes given data to a writer
    fn write_to(&mut self, w: &mut dyn Write, offset: u64, size: usize) -> Result<(), Error> {
        const CAPACITY: usize = 100 * 1024 * 1024; // Set temp buffer capacity to 100MB
        let mut buffer = vec![0; CAPACITY]; // allocate an empty buffer until the specified capacity
        let mut bytes_copied = 0;
        //eprintln!("seeking the right offset");
        self.data.seek(SeekFrom::Start(offset))?;
        //let mut read_buffer = self.data.take(part.header.filesize());
        // Buffered copy to the output file
        while bytes_copied < size {
            let remaining_bytes = size - bytes_copied;
            //eprintln!("remaining_bytes = {remaining_bytes}");
            // shrink buffer to the right size
            buffer.truncate(std::cmp::min(CAPACITY, remaining_bytes));
            //eprintln!("buffer len: {}", buffer.len());
            //eprintln!("buffer capacity: {}", buffer.capacity());
            //eprintln!("filling buffer...");
            let bytes_read = self.data.read(&mut buffer)?;
            //eprintln!("read {bytes_read} bytes");
            if bytes_read == 0 {
                return Err(Error::new("Read 0 bytes".into()));
            }
            w.write_all(&buffer)?;
            //eprintln!("Done copying");
            bytes_copied += bytes_read;
            //eprintln!("bytes_copied = {bytes_copied}");
        }
        Ok(())
    }
}
