use std::fs;

fn main() -> std::io::Result<()> {
    // Specify the file path you want to delete
    let file_path = "example.txt";

    // Attempt to delete the file
    fs::remove_file(file_path)?;

    println!("File '{}' successfully deleted.", file_path);

    Ok(())
}
