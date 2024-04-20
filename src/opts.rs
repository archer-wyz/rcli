use ::clap::Parser;
use anyhow::Result;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

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
}

#[derive(Debug, Clone, Copy)]

pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(short, long, value_parser = verity_input_file)]
    pub input: String,
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long, default_value = "json", value_parser = output_format_parse)]
    pub format: OutputFormat,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value = "16")]
    pub length: usize,
    #[arg(short, long, default_value = "1")]
    pub count: usize,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

fn verity_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

fn output_format_parse(format: &str) -> Result<OutputFormat, anyhow::Error> {
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
    fn from_str(format: &str) -> Result<Self, Self::Err> {
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
