use super::verify_duration;
use anyhow::Result;
use chrono::Duration;
use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum JwtSubCommand {
    #[command(about = "Sign JWT")]
    Sign(JwtSignOpts),
    #[command(about = "Verify JWT")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long, default_value = "1h", value_parser = verify_duration)]
    pub exp: Duration,
    #[arg(long, default_value = "HS256", value_parser = verify_jwt_alg)]
    pub alg: JwtAlg,
    #[arg(long, default_value = "y^sf+rIpfi^")]
    pub key: String,
}

fn verify_jwt_alg(value: &str) -> Result<JwtAlg> {
    value.parse()
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
    #[arg(long, default_value = "HS256", value_parser = verify_jwt_alg)]
    pub alg: JwtAlg,
    #[arg(long, default_value = "y^sf+rIpfi^")]
    pub key: String,
}

#[derive(Clone, Copy, Debug)]
pub enum JwtAlg {
    HS256,
    HS384,
}

impl FromStr for JwtAlg {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "HS256" => Ok(Self::HS256),
            "HS384" => Ok(Self::HS384),
            _ => Err(anyhow::anyhow!("Invalid JWT algorithm")),
        }
    }
}

impl std::fmt::Display for JwtAlg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtAlg::HS256 => write!(f, "HS256"),
            JwtAlg::HS384 => write!(f, "HS384"),
        }
    }
}

impl From<JwtAlg> for &'static str {
    fn from(alg: JwtAlg) -> Self {
        match alg {
            JwtAlg::HS256 => "HS256",
            JwtAlg::HS384 => "HS384",
        }
    }
}
