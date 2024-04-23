use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tower_http::services::ServeDir;
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
    let state = HttpServerState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_br()
        .precompressed_gzip()
        .precompressed_deflate()
        .precompressed_zstd();

    let router = Router::new()
        .route("/self/*path", get(file_handler))
        .nest_service("/tower", dir_service)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServerState {
            path: PathBuf::from("."),
        });
        let response = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(response.0, StatusCode::OK);
        assert!(response.1.contains("tower-http"));
    }
}
