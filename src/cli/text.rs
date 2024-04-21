use super::verity_input_file;
use clap::Parser;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign text")]
    Sign(SignOpts),
    #[command(about = "Verify text")]
    Verify(VerifyOpts),
}
#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short, long, default_value = "-", value_parser = verity_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long, default_value = "blake", value_parser = verify_crypt_format)]
    pub format: CryptFormat,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short, long, default_value = "-", value_parser = verity_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long)]
    pub signature: String,
    #[arg(short, long, default_value = "blake", value_parser = verify_crypt_format)]
    pub format: CryptFormat,
}

#[derive(Debug, Copy, Clone)]
pub enum CryptFormat {
    BlakeCrypt,
}

fn verify_crypt_format(format: &str) -> anyhow::Result<CryptFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for CryptFormat {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> anyhow::Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "blake" => Ok(CryptFormat::BlakeCrypt),
            v => unreachable!("Unsupported format: {:?}", v),
        }
    }
}

impl fmt::Display for CryptFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptFormat::BlakeCrypt => write!(f, "blake"),
        }
    }
}

impl From<CryptFormat> for &'static str {
    fn from(format: CryptFormat) -> Self {
        match format {
            CryptFormat::BlakeCrypt => "blake",
        }
    }
}
