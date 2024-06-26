use std::fs::File;
use std::io::Read;

pub mod base64_ed;
pub mod csv_convert;
pub mod gen_pass;
pub mod http_serve;
pub(crate) mod jwt;
mod text;
pub mod text_op;

fn data_from_input(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
