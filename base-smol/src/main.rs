//! A simple HTTP server.
//!
//! Run with:
//!
//! ```
//! cargo run --example simple-server
//! ```
//!
//! Open in the browser any of these addresses:
//!
//! - http://localhost:8000/
//! - https://localhost:8001/ (accept the security prompt in the browser)
//!
//! Refer to `README.md` to see how to the TLS certificate was generated.
//!
//! Reference:
//! https://github.com/smol-rs/smol/blob/master/examples/simple-server.rs
//!
use std::net::{TcpListener, TcpStream};

use anyhow::Result;
use smol::{prelude::*, Async};

const RESPONSE: &[u8] = br#"
HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 47
<!DOCTYPE html><html><body>Hello, World!</body></html>
"#;

/// Reads a request from the client and sends it a response.
async fn serve(mut stream: Async<TcpStream>) -> Result<()> {
    stream.write_all(RESPONSE).await?;
    Ok(())
}

/// Listens for incoming connections and serves them.
async fn listen(listener: Async<TcpListener>) -> Result<()> {
    loop {
        // Accept the next connection.
        let (stream, _) = listener.accept().await?;
        // Spawn a background task serving this connection.
        smol::spawn(async move {
            if let Err(err) = serve(stream).await {
                println!("Connection error: {:#?}", err);
            }
        })
        .detach();
    }
}

fn main() -> Result<()> {
    // Start HTTP server.
    smol::block_on(async {
        let http = listen(Async::<TcpListener>::bind(([127, 0, 0, 1], 3000))?);
        http.await?;
        Ok(())
    })
}
