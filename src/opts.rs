use std::path::Path;
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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short,value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, default_value = "output.json")]
    pub output: String,

    #[arg(short, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}


fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        Err("File does not exist")
    }
}