use super::{output_format_parse, verity_input_file, OutputFormat};
use clap::Parser;

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
