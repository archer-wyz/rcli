use super::verity_input_file;
use clap::Parser;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign text")]
    Sign(SignOpts),
    #[command(about = "Verify text")]
    Verify(VerifyOpts),
    #[command(about = "Generate key")]
    Generate(GenerateOpts),
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

#[derive(Debug, Parser)]
pub struct GenerateOpts {
    #[arg(short, long, default_value = "blake", value_parser = verify_crypt_format)]
    pub format: CryptFormat,
    #[arg(short, long, default_value = ".", value_parser = verity_dir_exist)]
    pub output: PathBuf,
}

#[derive(Debug, Copy, Clone)]
pub enum CryptFormat {
    BlakeCrypt,
    Ed25519Crypt,
}

fn verity_dir_exist(dir: &str) -> anyhow::Result<PathBuf, anyhow::Error> {
    let dir = Path::new(dir);
    if dir.exists() {
        Ok(dir.to_path_buf())
    } else {
        Err(anyhow::anyhow!("Directory does not exist"))
    }
}

fn verify_crypt_format(format: &str) -> anyhow::Result<CryptFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for CryptFormat {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> anyhow::Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "blake" => Ok(CryptFormat::BlakeCrypt),
            "ed25519" => Ok(CryptFormat::Ed25519Crypt),
            v => unreachable!("Unsupported format: {:?}", v),
        }
    }
}

impl fmt::Display for CryptFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptFormat::BlakeCrypt => write!(f, "blake"),
            CryptFormat::Ed25519Crypt => write!(f, "ed25519"),
        }
    }
}

impl From<CryptFormat> for &'static str {
    fn from(format: CryptFormat) -> Self {
        match format {
            CryptFormat::BlakeCrypt => "blake",
            CryptFormat::Ed25519Crypt => "ed25519",
        }
    }
}
