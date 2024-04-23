use super::{verity_dir_exist, verity_input_file};
use crate::{
    process_text_decrypt, process_text_encrypt, process_text_generate, process_text_sign,
    process_text_verify, CmdExector,
};
use clap::Parser;
use std::fmt;
use std::path::PathBuf;
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

impl CmdExector for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opts) => Ok(opts.execute().await?),
            TextSubCommand::Verify(opts) => Ok(opts.execute().await?),
            TextSubCommand::Generate(opts) => Ok(opts.execute().await?),
            TextSubCommand::Encrypt(opts) => Ok(opts.execute().await?),
            TextSubCommand::Decrypt(opts) => Ok(opts.execute().await?),
        }
    }
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

impl CmdExector for SignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let result = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", result);
        Ok(())
    }
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

impl CmdExector for VerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let result = process_text_verify(&self.input, &self.key, &self.signature, self.format)?;
        println!("{}", result);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct GenerateOpts {
    #[arg(short, long, default_value = "blake", value_parser = verify_crypt_format)]
    pub format: CryptFormat,
    #[arg(short, long, default_value = ".", value_parser = verity_dir_exist)]
    pub output: PathBuf,
}

impl CmdExector for GenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let result = process_text_generate(self.format)?;
        for (key, value) in result {
            println!("{}: {:?}", key, value);
            let path = self.output.join(key);
            ::std::fs::write(path, value)?;
        }
        Ok(())
    }
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

impl CmdExector for EncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let result = process_text_encrypt(&self.input, &self.key, self.format)?;
        println!("{}", result);
        Ok(())
    }
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

impl CmdExector for DecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let result = process_text_decrypt(&self.input, &self.key, self.format)?;
        let result = String::from_utf8(result).map_err(|_| {
            anyhow::anyhow!("Decrypt successfully, but the payloads exist invalid UTF-8")
        })?;
        println!("{}", result);
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CryptFormat {
    ChaCha20Poly1305,
    Blake,
    Ed25519,
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
            CryptFormat::Blake => write!(f, "blake"),
            CryptFormat::Ed25519 => write!(f, "ed25519"),
        }
    }
}

impl From<CryptFormat> for &'static str {
    fn from(format: CryptFormat) -> Self {
        match format {
            CryptFormat::ChaCha20Poly1305 => "chacha20poly1305",
            CryptFormat::Blake => "blake",
            CryptFormat::Ed25519 => "ed25519",
        }
    }
}
