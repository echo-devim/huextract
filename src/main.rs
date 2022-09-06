use clap::Parser;
use extractor::Extractor;
use local_error::Error;

mod crc;
mod extractor;
mod img;
mod img_header;
mod input;
mod local_error;
mod tabled_types;
mod utils;

fn main() -> Result<(), Error> {
    let extractor = Extractor::parse();

    extractor.run()
}
