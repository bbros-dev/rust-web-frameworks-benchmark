#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 3000))
        .merge(("address", "127.0.0.1"));
    rocket::custom(figment).mount("/", routes![hello])
}
