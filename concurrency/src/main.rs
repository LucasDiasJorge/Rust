// File: src/main.rs

use std::fs::File;
use std::io::{self, Read};
use std::thread;
use std::sync::{Arc, Mutex};
use reqwest;

fn main() {
    // URLs to download concurrently
    let urls = vec![
        "https://example.com/file1.txt",
        "https://example.com/file2.txt",
        // Add more URLs as needed
    ];

    // Shared state for storing downloaded content
    let downloaded_data = Arc::new(Mutex::new(Vec::new()));

    // Create a vector to store thread handles
    let mut handles = vec![];

    for url in urls {
        // Clone the Arc for each thread
        let downloaded_data_clone = Arc::clone(&downloaded_data);

        // Spawn a new thread
        let handle = thread::spawn(move || {
            // Download the file content
            if let Ok(data) = download_file(url) {
                // Lock the mutex and update the shared downloaded_data
                let mut downloaded_data = downloaded_data_clone.lock().unwrap();
                downloaded_data.push(data);
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Access the final downloaded data
    let final_data = downloaded_data.lock().unwrap();
    for (index, data) in final_data.iter().enumerate() {
        save_to_file(index, data);
    }
}

fn download_file(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    // Placeholder for the file download logic (using reqwest)
    reqwest::blocking::get(url)?.bytes().map(|bytes| bytes.to_vec())
}

fn save_to_file(index: usize, data: &[u8]) {
    // Placeholder for saving downloaded data to a file
    let filename = format!("file_{}.bin", index);
    let mut file = File::create(&filename).expect("Error creating file");
    file.write_all(data).expect("Error writing to file");
    println!("File {} saved successfully.", filename);
}
