use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::thread;

struct ThreadPool;

impl ThreadPool {
    fn new(_size: usize) -> ThreadPool {
        // Your ThreadPool initialization logic goes here
        ThreadPool
    }

    fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Your execution logic goes here
        thread::spawn(_f);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Assuming request_line is defined or obtained earlier in your code
    let request_line = "GET /sleep HTTP/1.1"; // Replace this with the actual request_line

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            println!("Entry");
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            String::from("File not found")
        }
    };
    
    let response = format!("{}\r\n\r\n{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
