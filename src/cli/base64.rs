use clap::Parser;

#[derive(Debug, Parser)]
pub struct Base64Opts {
    pub encode: bool,
    pub decode: bool,
    pub input: String,
}
