use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use std::fmt;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tera::Context;
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug)]
struct HttpServerState {
    path: PathBuf,
    index_template: tera::Tera,
}

pub async fn process_http_serve(
    path: PathBuf,
    address: &str,
    port: u32,
    security: bool,
) -> Result<()> {
    let address = format!("{}:{}", address, port);
    info!("http serve: {:?} {} {}", path, address, security);

    let template = tera::Tera::new("./assets/templates/*")?;
    let state = HttpServerState {
        path: path.clone(),
        index_template: template,
    };

    let dir_service = ServeDir::new(path.clone())
        .precompressed_br()
        .precompressed_gzip()
        .precompressed_deflate()
        .precompressed_zstd();

    let router = Router::new()
        .route("/self", get(dir_file_handler_base))
        .route("/self/*path", get(dir_file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(state));
    let addr = SocketAddr::from_str(&address).map_err(|_| anyhow::anyhow!("Invalid address"))?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn dir_file_handler(
    State(state): State<Arc<HttpServerState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let p = std::path::Path::new(&state.path).join(path);
    _dir_file_handler(p, state).await
}

async fn dir_file_handler_base(State(state): State<Arc<HttpServerState>>) -> impl IntoResponse {
    let p = std::path::Path::new(&state.path).join(".");
    _dir_file_handler(p, state).await
}

async fn _dir_file_handler(path: PathBuf, state: Arc<HttpServerState>) -> impl IntoResponse {
    let p_type: PathType = path.clone().into();
    match p_type {
        PathType::File => match tokio::fs::read_to_string(&path).await {
            Ok(val) => {
                info!("serving file len: {}", val.len());
                (StatusCode::OK, val).into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error reading file: {}", e),
            )
                .into_response(),
        },
        PathType::Directory => match tokio::fs::read_dir(&path).await {
            Ok(mut reader) => {
                let mut files = Vec::new();
                loop {
                    match reader.next_entry().await {
                        Ok(Some(entry)) => {
                            files.push(entry.file_name());
                        }
                        Ok(None) => break,
                        Err(e) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Error reading directory: {}", e),
                            )
                                .into_response()
                        }
                    }
                }
                let mut context = Context::new();
                let files = files
                    .iter()
                    .map(|f| f.to_str().unwrap())
                    .collect::<Vec<&str>>();
                context.insert("directory_name", path.to_str().unwrap());
                context.insert("files", &files);
                match state
                    .index_template
                    .render("directory_index.html", &context)
                {
                    Ok(html) => Html(html).into_response(),
                    Err(e) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Error rendering template: {}", e),
                    )
                        .into_response(),
                }
            }
            Err(e) => Html(format!("Error reading directory: {}", e)).into_response(),
        },
        PathType::Unknown => (StatusCode::NOT_FOUND, "Path not found".to_string()).into_response(),
    }
}

#[derive(Debug)]
enum PathType {
    File,
    Directory,
    Unknown,
}

impl From<PathBuf> for PathType {
    fn from(value: PathBuf) -> Self {
        if value.is_file() {
            PathType::File
        } else if value.is_dir() {
            PathType::Directory
        } else {
            PathType::Unknown
        }
    }
}

impl fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathType::File => write!(f, "file"),
            PathType::Directory => write!(f, "dir"),
            PathType::Unknown => write!(f, "unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServerState {
            path: PathBuf::from("."),
            index_template: tera::Tera::new("./assets/templates/*").unwrap(),
        });
        dir_file_handler(State(state), Path("Cargo.toml".to_string())).await;
    }

    #[tokio::test]
    async fn test_dir_handler() {
        let state = Arc::new(HttpServerState {
            path: PathBuf::from("."),
            index_template: tera::Tera::new("./assets/templates/*").unwrap(),
        });
        dir_file_handler(State(state), Path("src".to_string())).await;
    }
}
