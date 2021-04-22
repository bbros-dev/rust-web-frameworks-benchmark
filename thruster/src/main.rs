// Reference:
// - https://github.com/thruster-rs/Thruster#middleware-based
use std::boxed::Box;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

use thruster::{App, BasicContext as Ctx, Request};
use thruster::{async_middleware, middleware_fn, MiddlewareNext, MiddlewareResult, Server, ThrusterServer};

#[middleware_fn]
async fn profile(context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let start_time = Instant::now();

    context = next(context).await;

    let elapsed_time = start_time.elapsed();
    println!(
        "[{}μs] {} -- {}",
        elapsed_time.as_micros(),
        context.request.method(),
        context.request.path()
    );

    Ok(context)
}

#[middleware_fn]
async fn plaintext(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "Hello, World!";
    context.body(val);
    Ok(context)
}

#[middleware_fn]
async fn four_oh_four(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.status(404);
    context.body("Whoops! That route doesn't exist!");
    Ok(context)
}

#[tokio::main]
fn main() {
    println!("Starting server...");

    let mut app = App::<Request, Ctx>::new_basic();

    app.get("/plaintext", async_middleware!(Ctx, [profile, plaintext]));
    app.set404(async_middleware!(Ctx, [four_oh_four]));

    let server = Server::new(app);
    server.build("0.0.0.0", 4321).await;
}
