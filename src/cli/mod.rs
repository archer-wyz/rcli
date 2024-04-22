mod base64;
mod csv;
mod gen_pass;
mod http;
mod text;

use clap::Parser;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub use self::{base64::*, csv::*, gen_pass::*, http::*, text::*};
#[derive(Debug, Parser)]
#[clap(name = "rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

fn verity_input_file(filename: &str) -> anyhow::Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

fn verity_dir_exist(dir: &str) -> anyhow::Result<PathBuf, anyhow::Error> {
    let dir = Path::new(dir);
    if dir.exists() {
        Ok(dir.to_path_buf())
    } else {
        Err(anyhow::anyhow!("Directory does not exist"))
    }
}

fn output_format_parse(format: &str) -> anyhow::Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> anyhow::Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            v => anyhow::bail!("Unsupported output format: {}", v),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert!(verity_input_file("-").is_ok());
        assert!(verity_input_file("Cargo.toml").is_ok());
        assert!(verity_input_file("unknown").is_err());
    }
}
