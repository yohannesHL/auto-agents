// Minimal Rust stub service included to make the monorepo genuinely polyglot
// (TypeScript, Python, Go, Rust).
// Replace this with a real Rust agent or gateway integration as needed.

use std::env;
use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let port = env::var("RUST_SERVICE_PORT").unwrap_or_else(|_| "8081".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&addr)
        .unwrap_or_else(|e| { eprintln!("rust-service: bind failed: {}", e); std::process::exit(1) });

    println!("rust-service listening on {}", addr);

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buf = [0u8; 1024];
            let _ = stream.read(&mut buf);
            let body = "rust-service stub — replace with your Rust agent\n";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
        }
    }
}
