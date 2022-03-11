fn main() {
    trillium_smol::run(|conn: trillium::Conn| async move {
        conn.ok("Hello, World!")
    });
}
