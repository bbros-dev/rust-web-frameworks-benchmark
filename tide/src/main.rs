#[async_std::main]
fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new(());
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.listen("127.0.0.1:3000").await?;
    Ok(())
}
