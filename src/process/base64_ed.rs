use crate::cli::Base64Format;
use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::prelude::*;
use std::{fs::File, io::Read};

pub fn process_base64_encode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = data_from_input(input)?;
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;

    let ret = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(data),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(data),
    };
    Ok(ret)
}

fn data_from_input(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = data_from_input(input)?;
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let data = data.trim();
    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(data),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(data),
    }?;
    String::from_utf8(decoded).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_base64_encode() {
        let input = "fixtures/base64_test/helloworld";
        let encoded = process_base64_encode(input, Base64Format::Standard).unwrap();
        assert_eq!(encoded, "aGVsbG8gd29ybGQhCg==");
    }

    #[test]
    fn test_process_base64_decode() {
        let input = "fixtures/base64_test/helloworld_b64";
        let decoded = process_base64_decode(input, Base64Format::Standard).unwrap();
        assert_eq!(decoded, "hello world!\n");
    }
}
