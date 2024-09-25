use super::verify_input_file;
use clap::Parser;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "base64 encode")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "base64 decode")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long,value_parser = verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format,default_value =  "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long,value_parser = verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format,default_value =  "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Copy, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    match format {
        "standard" => Ok(Base64Format::Standard),
        "urlsafe" => Ok(Base64Format::UrlSafe),
        _ => Err(anyhow::anyhow!("Invalid base64 format: {}", format)),
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_base64_format(s)
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "urlsafe"),
        }
    }
}

impl From<Base64Format> for String {
    fn from(format: Base64Format) -> Self {
        format.to_string()
    }
}
