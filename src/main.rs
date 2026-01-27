mod args;
use arboard::Clipboard;
use axum::{Router, extract::State, http::StatusCode, response::Response, routing::get};
use axum::{body::Body, http::header};
use clap::Parser;
use local_ip_address::local_ip;
use std::net::SocketAddr;
use std::path::Path;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();

    let file_path = args.file.clone().unwrap_or_default();

    if !args.upload {
        if !Path::new(&file_path).exists() {
            eprintln!("Error: The file '{}' does not exist.", file_path);
            std::process::exit(1);
        }
    }

    let lan_ip = local_ip().expect("Failed to determine local IP address");
    let path = format!("/{}", Uuid::new_v4());
    let display_addr = SocketAddr::new(lan_ip, args.port);
    let bind_addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    let url = format!("http://{display_addr}{path}");

    let app = Router::new()
        .route(&path, get(handler))
        .with_state(file_path);

    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();

    if args.file.is_some() {
        println!("File reachable under:\n{url}");
        if args.copy {
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(url).unwrap();
        }
    }

    if args.upload {
        println!("Listening for uploads on port {}", args.port);
    }

    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(file_path): State<String>) -> Result<Response<Body>, (StatusCode, String)> {
    let file = tokio::fs::File::open(&file_path).await.map_err(|err| {
        (
            StatusCode::NOT_FOUND,
            format!("File not found ({}): {}", file_path, err),
        )
    })?;

    let filename = Path::new(&file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("download");

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Response::builder()
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(body)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}
