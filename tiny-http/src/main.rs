// References:
// - https://github.com/tiny-http/tiny-http#usage

use tiny_http::{Server, Response};

let server = Server::http("0.0.0.0:8000").unwrap();

for request in server.incoming_requests() {
    let response = Response::from_string("hello world");
    request.respond(response);
}
