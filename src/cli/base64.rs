use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use super::verify_input_file;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode base64")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "decode base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long,value_parser = verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(long, default_value = "standard",value_parser=parse_base_format)]
    pub format: Base64Format,
}

#[derive(Debug, Copy, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(long, default_value = "standard",value_parser=parse_base_format)]
    pub format: Base64Format,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Unknown format: {}", s)),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

fn parse_base_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}