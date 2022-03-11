// Reference:
// -
use salvo::prelude::*;

#[fn_handler]
async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .get(hello_world);
    Server::new(router).bind(([127, 0, 0, 1], 3000)).await;
}
