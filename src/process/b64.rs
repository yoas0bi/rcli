use crate::Base64Format;
use base64::engine::general_purpose::STANDARD;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use std::fs::File;
use std::io::{stdin, Read};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    println!("{:?}", buf);
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE.encode(buf),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    println!("{:?}", buf);
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE.decode(buf)?,
    };
    println!("{}", String::from_utf8(decoded)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_encode() {
        let input = "-";
        let format = Base64Format::UrlSafe;
        process_encode(input, format).unwrap();

        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        process_encode(input, format).unwrap();
    }

    #[test]
    fn test_process_decode() {
        let input = "assets/b64.txt";
        let format = Base64Format::Standard;
        process_decode(input, format).unwrap();
    }
}
