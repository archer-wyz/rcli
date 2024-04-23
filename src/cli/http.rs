use super::verity_dir_exist;
use crate::{process_http_serve, CmdExector};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serv the http")]
    Serve(HttpOpts),
}

impl CmdExector for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => opts.execute().await?,
        }
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct HttpOpts {
    #[arg(short, long, default_value_t = true)]
    pub security: bool,
    #[arg(short, long, default_value = ".", value_parser = verity_dir_exist)]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "0.0.0.0")]
    pub address: String,
    #[arg(short, long, default_value = "8080", value_parser = parse_port)]
    pub port: u32,
}

fn parse_port(port: &str) -> anyhow::Result<u32, &'static str> {
    match port.parse() {
        Ok(port) => {
            if port > 0 && port < 65536 {
                Ok(port)
            } else {
                Err("Port must be between 1 and 65535")
            }
        }
        Err(_) => Err("Port must be a number"),
    }
}

impl CmdExector for HttpOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, &self.address, self.port, self.security).await
    }
}
