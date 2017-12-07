extern crate num_cpus;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::thread;
use std::io::Cursor;
use std::io::BufReader;
use std::io::BufWriter;

/* Huawei UPDATE.APP data structure:
1. First 92 bytes are 0x00
2. Each file are started with 55AA 5AA5
3. Then 4 bytes for Header Length
4. Then 4 bytes for Unknown1
5. Then 8 bytes for Hardware ID
6. Then 4 bytes for File Sequence
7. Then 4 bytes for File Size
8. Then 16 bytes for File Date
9. Then 16 bytes for File Time
10.Then 16 bytes for File Type
11.Then 16 bytes for Blank1
12.Then 2 bytes for Header Checksum
13.Then 2 bytes for BlockSize
14.Then 2 bytes for Blank2
15.Then ($headerLength-98) bytes for file checksum
16.Then data file length bytes for files.
17.Then padding if have
18.Then repeat 2 to 17
Thanks to: https://github.com/marcominetti/split_updata.pl
*/
#[derive(Default)]
struct BlockHeader {
    file_size: [u8; 4],
    file_type: [u8; 16]
}

fn thread_extract(mut file: File, offset: u64, filename: &String, filesize: usize) {
    //Skip some uninteresting header entries
    let mut out_file = File::create(filename).unwrap();
    let limit = 100*1024*1024; //100MB, decrease this number to save memory
    let mut size: usize = limit; //read using 100MB chunks
    if (filesize < limit) {
        size = filesize;
    }
    file.seek(SeekFrom::Start(offset + 100)); //Skip the header block
    let mut reader = BufReader::with_capacity(size, file);
    let mut missing = filesize;
    loop {
        let length = {
            let buffer = reader.fill_buf().unwrap();
            if (size > missing) {
                out_file.write_all(&buffer[..missing]);
                break;
            } else {
                out_file.write_all(buffer);
                missing = missing - size;
            }
            size
        };
        reader.consume(length);
    }
    println!("{} extracted!", filename);
}

fn remove_null_bytes(buffer: [u8; 16]) -> Vec<u8> {
    let mut pos = 0;
    for byte in buffer.to_vec() {
        if byte == 0x00 {
            break;
        }
        pos += 1;
        if (pos > 16) { break; }
    }
    let mut new_buffer = vec![0; pos];
    new_buffer.clone_from_slice(&buffer[..pos]);
    new_buffer
}

fn extract(update_file: String) {
    let cpus = num_cpus::get();
    let mut threads = vec![];
    let block_signature = [0x55, 0xAA, 0x5A, 0xA5];
    let mut buffer = [0; 4];
    let mut uf = File::open(&update_file).expect("Update file not found");
    //Loop to findand extract all the files
    let end = uf.seek(SeekFrom::End(0)).unwrap();
    println!("Total size: {} bytes", end);
    //skip the first 92 0x00 bytes
    let mut offset = 92;
    uf.seek(SeekFrom::Start(offset));
    loop {
        uf.read_exact(&mut buffer);
        //If a file is found..
        if (buffer == block_signature) {
            println!("Found file block at 0x{:x}", offset);
            //Header analysis
            //Skip some uninteresting header entries
            uf.seek(SeekFrom::Start(offset + 24));
            let mut bh = BlockHeader {..Default::default()};
            uf.read_exact(&mut bh.file_size);
            uf.seek(SeekFrom::Current(32));
            uf.read_exact(&mut bh.file_type);
            let filename = match String::from_utf8(remove_null_bytes(bh.file_type)) {
                Ok(filename) => filename + ".img",
                Err(_) => { println!("Invalid file at 0x{:x}, ignoring it..", offset); return}
            };
            let filesize = unsafe {std::mem::transmute::<[u8; 4], u32>(bh.file_size) }.to_le() as u64;
            println!("Extracting {} ({} bytes)", filename, filesize);
            //Start a new thread to do the job
            //Each thread should work on a different file handler for safety reasons
            let uf_copy = File::open(&update_file).expect("Update file not found");
            let handle = thread::spawn(move || thread_extract(uf_copy, offset, &filename, filesize as usize));
            threads.push(handle);
            //wait if there are too many running threads
            if (threads.len() >= cpus - 1) {
                let handle = threads.remove(0);
                handle.join();
            }

            offset += filesize; //go to the next block (not accurate)
        }
        //Move forward (1 byte at time) ignoring the previous 4 movements (curr_pos - 4 + 1)
        if (uf.seek(SeekFrom::Start(offset+1)).unwrap() > end) {
            break;
        } else {
            offset += 1;
        }
    }

    for t in threads {
        t.join();
    }
}

fn main() {
    let update_file = env::args().nth(1).expect("Missing 1 argument: update.app file path");
    println!("Extracting files from {}", update_file);
    extract(update_file);
    println!("Finished");
}
