use ::clap::Parser;
use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(short, long, value_parser = verity_input_file)]
    pub input: String,
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
}

fn verity_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}
