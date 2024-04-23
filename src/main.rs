use ::anyhow;
use ::clap::Parser;
use rcli::{CmdExector, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    tracing_subscriber::fmt().init();
    opts.cmd.execute().await?;
    Ok(())
}
