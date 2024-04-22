use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

#[derive(Debug)]
struct HttpServerState {
    path: PathBuf,
}

pub async fn process_http_serve(
    path: PathBuf,
    address: &str,
    port: u32,
    security: bool,
) -> Result<()> {
    let address = format!("{}:{}", address, port);
    info!("http serve: {:?} {} {}", path, address, security);
    let state = HttpServerState { path };
    let router = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let addr = SocketAddr::from_str(&address).map_err(|_| anyhow::anyhow!("Invalid address"))?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServerState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    if !p.exists() {
        return (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        );
    }
    match tokio::fs::read_to_string(p).await {
        Ok(val) => {
            info!("serving file len: {}", val.len());
            (StatusCode::OK, val)
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error reading file: {}", e),
        ),
    }
}
