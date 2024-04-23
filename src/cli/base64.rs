use super::verity_input_file;
use crate::{process_base64_decode, process_base64_encode, CmdExector};
use anyhow;
use clap::Parser;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Base64 encode")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Base64 decode")]
    Decode(Base64DecodeOpts),
}

impl CmdExector for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => {
                let result = process_base64_encode(&opts.input, opts.format)?;
                println!("{}", result);
            }
            Base64SubCommand::Decode(opts) => {
                let result = process_base64_decode(&opts.input, opts.format)?;
                println!("{}", result);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verity_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long, value_parser = verify_base64_yaml_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verity_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long, value_parser = verify_base64_yaml_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Copy, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn verify_base64_yaml_format(format: &str) -> anyhow::Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> anyhow::Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            v => unreachable!("Unsupported format: {:?}", v),
        }
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

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}
