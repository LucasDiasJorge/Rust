use std::fs;

fn main() -> std::io::Result<()> {
    let entries = fs::read_dir(".")?;
    for entry in entries {
        if let Ok(entry) = entry {
            println!("Name: {}", entry.file_name().to_string_lossy());
        }
    }
    Ok(())
}
