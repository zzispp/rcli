mod base64;
mod csv;
mod genpass;
mod text;

pub use self::base64::{Base64SubCommand, Base64Format};
pub use self::csv::OutputFormat;
pub use self::text::{TextSignFormat,TextSubCommand};
use std::path::{Path, PathBuf};

use crate::cli::csv::CsvOpts;
use crate::cli::genpass::GenPassOpts;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rcli",version = "0.0.1",author,about="Rust Cli ",long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Rust CLI for CSV files.")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "gen password CLI")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),

    #[command(subcommand)]
    Text(TextSubCommand)
}

pub fn verify_file(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        Err("File does not exist")
    }
}


fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("Cargo1.toml"), Err("File does not exist"));
    }
}
