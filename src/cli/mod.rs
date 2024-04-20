mod base64;
mod csv;
mod gen_pass;

use clap::Parser;

pub use self::{base64::*, csv::*, gen_pass::*};
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
    #[command(name = "base64", about = "Base64 encode/decode")]
    Base64(Base64Opts),
}
