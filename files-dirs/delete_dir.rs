use std::fs;

fn main() -> std::io::Result<()> {
    fs::remove_dir("directory_to_delete")?;
    Ok(())
}
