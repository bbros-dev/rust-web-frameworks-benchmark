//! References:
//! - https://github.com/tiny-http/tiny-http/blob/master/examples/hello-world.rs
//!
extern crate tiny_http;

use std::sync::Arc;
use std::thread;

fn main() {
    let server = Arc::new(tiny_http::Server::http("127.0.0.1:3000").unwrap());

    let mut handles = Vec::new();

    for _ in 0..4 {
        let server = server.clone();

        handles.push(thread::spawn(move || {
            for rq in server.incoming_requests() {
                let response = tiny_http::Response::from_string("hello world".to_string());
                let _ = rq.respond(response);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
