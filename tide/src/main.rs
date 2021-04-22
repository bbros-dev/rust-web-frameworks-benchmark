// Reference:
// -
#![feature(async_await)]
fn main() {
    let mut app = tide::App::new(());
    let app_config = Configuration::build()
        .address(String::from("0.0.0.0"))
        .port(3002)
        .finalize();
    app.at("/").get(async || "Hello, world!");
    app.serve();
}
