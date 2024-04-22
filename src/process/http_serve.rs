use anyhow::Result;
use std::path::Path;
use tracing::info;

pub fn process_http_serve(path: &Path, address: &str, port: u32, security: bool) -> Result<()> {
    let address = if security {
        format!("https://{}:{}", address, port)
    } else {
        format!("http://{}:{}", address, port)
    };
    info!("http serve: {:?} {}", path, address);
    Ok(())
}
