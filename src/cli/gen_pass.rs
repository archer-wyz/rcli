use crate::{process_gen_pass, CmdExector};
use clap::Parser;

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

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let passwords = process_gen_pass(
            self.length,
            self.count,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        for password in passwords {
            println!("{}", password);
        }
        Ok(())
    }
}
