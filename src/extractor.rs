//! This module contains the Extractor implementation, which is the core logic
//! of the program.
//!
use clap::{Args, Parser, Subcommand};

use std::convert::TryFrom;
use std::path::PathBuf;

use crate::input::Input;
use crate::local_error::Error;

/// Extract the information contained in an UPDATE.APP file from Huawei smartphone
/// firmwares.
#[derive(Parser)]
pub struct Extractor {
    /// The command to execute.
    ///
    /// Defaults to extract-imgs.
    #[clap(subcommand)]
    command: ExtractorCommand,
    /// The name of the file to extract the img files from.
    ///
    /// Defaults to UPDATE.APP.
    #[clap(value_parser, default_value_os_t = PathBuf::from("UPDATE.APP"))]
    input: PathBuf,
    /*    /// Show content of file instead of extracting.
    #[clap(short = 'C', long, group = "action")]
    show_content: bool,
    /// Show header summary instead of extracting.
    #[clap(short = 'H', long, group = "action")]
    show_headers: bool,
    /// Dump header table into a parseable file.
    #[clap(short, long, group = "action")]
    dump_headers: bool,
    /// Extract the img files
    #[clap(short, long, group = "action")]
    extract_img: bool,
    /// Extract the checksum of the img files
    #[clap(short = 'S', long, group = "action")]
    extract_checksum: bool,*/
}

/// The command to execute.
///
/// Defaults to extract-imgs.
#[derive(Subcommand)]
enum ExtractorCommand {
    /// Extract the img files contained in the input file.
    Extract(Extract),
    /// List the img files contained in the input file.
    List,
    /// Extract the raw content of the headers into files.
    ExtractHeaders,
    /// Show a summary of the headers content.
    ShowHeaders,
    /// Extract only the file checksums.
    ExtractChecksums,
    /// Export the headers content into a CSV file.
    ExportHeadersCsv,
}

impl Default for ExtractorCommand {
    fn default() -> Self {
        ExtractorCommand::Extract(Extract::default())
    }
}

#[derive(Args, Default)]
struct Extract {
    /// Don't verify checksum for extracted files.
    #[clap(short, long)]
    no_checksum_verification: bool,
}

impl Extractor {
    pub fn run(self) -> Result<(), Error> {
        println!("Using input file {}", self.input.display());
        if !self.input.exists() {
            Err(Error::from(format!(
                "File {} does not exist",
                self.input.display()
            )))
        } else {
            let mut input = Input::try_from(self.input.as_path())?;

            input.validate()?;

            // Parse the input to get img headers
            input.parse()?;

            match self.command {
                //.unwrap_or(DEFAULT_COMMAND) {
                ExtractorCommand::List => println!("{input}"),
                ExtractorCommand::ShowHeaders => println!("{}", input.full_table()),
                ExtractorCommand::ExportHeadersCsv => println!("{}", input.export_csv()),
                ExtractorCommand::Extract(options) => {
                    input.extract_img(options.no_checksum_verification)?
                }
                ExtractorCommand::ExtractChecksums => input.extract_checksum()?,
                ExtractorCommand::ExtractHeaders => unimplemented!(),
            }

            Ok(())
        }
    }
}
