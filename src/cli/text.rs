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
    #[command(about = "Encrypt text")]
    Encrypt(EncryptOpts),
    #[command(about = "Decrypt text")]
    Decrypt(DecryptOpts),
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
    pub format: SigOrVerFormat,
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
    #[arg(short, long, default_value = "blake", value_parser = verify_sig_or_ver_format)]
    pub format: SigOrVerFormat,
}

#[derive(Debug, Parser)]
pub struct GenerateOpts {
    #[arg(short, long, default_value = "blake", value_parser = verify_sig_or_ver_format)]
    pub format: SigOrVerFormat,
    #[arg(short, long, default_value = ".", value_parser = verity_dir_exist)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct EncryptOpts {
    #[arg(short, long, default_value = "-", value_parser = verity_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long, default_value = "chacha20poly1305", value_parser = verify_crypt_format)]
    pub format: CryptFormat,
}

#[derive(Debug, Parser)]
pub struct DecryptOpts {
    #[arg(short, long, default_value = "-", value_parser = verity_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long, default_value = "chacha20poly1305", value_parser = verify_crypt_format)]
    pub format: CryptFormat,
}

#[derive(Debug, Copy, Clone)]
pub enum SigOrVerFormat {
    Blake,
    Ed25519,
}

#[derive(Debug, Copy, Clone)]
pub enum CryptFormat {
    ChaCha20Poly1305,
}

fn verity_dir_exist(dir: &str) -> anyhow::Result<PathBuf, anyhow::Error> {
    let dir = Path::new(dir);
    if dir.exists() {
        Ok(dir.to_path_buf())
    } else {
        Err(anyhow::anyhow!("Directory does not exist"))
    }
}

fn verify_sig_or_ver_format(format: &str) -> anyhow::Result<SigOrVerFormat, anyhow::Error> {
    format.parse()
}

fn verify_crypt_format(format: &str) -> anyhow::Result<CryptFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for CryptFormat {
    type Err = anyhow::Error;

    fn from_str(format: &str) -> anyhow::Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "chacha20poly1305" => Ok(CryptFormat::ChaCha20Poly1305),
            v => unreachable!("Unsupported format: {:?}", v),
        }
    }
}

impl fmt::Display for CryptFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptFormat::ChaCha20Poly1305 => write!(f, "chacha20poly1305"),
        }
    }
}

impl From<CryptFormat> for &'static str {
    fn from(format: CryptFormat) -> Self {
        match format {
            CryptFormat::ChaCha20Poly1305 => "chacha20poly1305",
        }
    }
}

impl FromStr for SigOrVerFormat {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> anyhow::Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "blake" => Ok(SigOrVerFormat::Blake),
            "ed25519" => Ok(SigOrVerFormat::Ed25519),
            v => unreachable!("Unsupported format: {:?}", v),
        }
    }
}

impl fmt::Display for SigOrVerFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigOrVerFormat::Blake => write!(f, "blake"),
            SigOrVerFormat::Ed25519 => write!(f, "ed25519"),
        }
    }
}

impl From<SigOrVerFormat> for &'static str {
    fn from(format: SigOrVerFormat) -> Self {
        match format {
            SigOrVerFormat::Blake => "blake",
            SigOrVerFormat::Ed25519 => "ed25519",
        }
    }
}
