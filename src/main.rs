use std::process::ExitCode;

use clap::{CommandFactory, Parser};
use extractor::Extractor;

mod crc;
mod extractor;
mod img;
mod img_header;
mod input;
mod local_error;
mod tabled_types;
mod utils;

fn main() -> ExitCode {
    let extractor = Extractor::parse();
    if let Err(e) = extractor.run() {
        // add extra line to improve readability
        println!("{e}\n");
        // print help message on error, this shouldn't fail
        Extractor::command().print_long_help().unwrap();
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
