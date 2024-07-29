use std::fs;

fn main() -> std::io::Result<()> {
    let content = "Hello, Rust!";
    fs::write("example.txt", content)?;
    Ok(())
}
