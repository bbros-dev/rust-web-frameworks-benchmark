//! An HTTP server based on `smol` minimal example, using `async-h1`.
//!
//! Run with:
//!
//! ```
//! cargo run -q --release --bin base-smol
//! ```
//!
//! Open in the browser:
//!
//! - http://localhost:3000/
//!
//! Reference:
//! https://github.com/smol-rs/smol/blob/master/examples/async-h1-server.rs
//!
use std::net::TcpListener;

use anyhow::Result;
use http_types::{Request, Response, StatusCode};
use smol::Async;

/// Serves a request and returns a response.
async fn serve(_req: Request) -> http_types::Result<Response> {
    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/plain");
    res.set_body("Hello, World!");
    Ok(res)
}

/// Listens for incoming connections and serves them.
async fn listen(listener: Async<TcpListener>) -> Result<()> {
    loop {
        // Accept the next connection.
        let (stream, _) = listener.accept().await?;

        // Spawn a background task serving this connection. See smol/issues/237
        let stream = async_dup::Arc::new(stream);
        let ex = smol::Executor::new();
        ex.spawn(async move {
            if let Err(err) = async_h1::accept(stream, serve).await {
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
