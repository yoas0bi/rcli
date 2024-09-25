mod base64;
mod csv;
mod genpass;

use clap::Parser;
use std::path::Path;

pub use self::base64::{Base64Format, Base64SubCommand};
pub use self::csv::{CsvOpts, OutputFormat};
pub use genpass::GenPassOpts;

#[derive(Debug, Parser)]
#[command(name = "rcli", version = "1.0", author = "yang", about = "convert csv to json", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "convert csv to json")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

pub fn verify_input_file(filename: &str) -> Result<String, String> {
    //if input is "-" or not exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("{}: No such file or directory", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_input_file_test() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_input_file("a.txt"),
            Err("a.txt: No such file or directory".into())
        );
    }
}
