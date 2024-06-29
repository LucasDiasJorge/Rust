use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

struct HttpHeader {
    method: String,
    path: String,
    protocol: String
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if let Some(first_line) = http_request.get(0) {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() == 3 {
            let method = parts[0].to_string();
            let path = parts[1].to_string();
            let protocol = parts[2].to_string();

            let http_header = HttpHeader { method, path, protocol};

            println!("Method: {}", http_header.method);
            println!("Path: {}", http_header.path);
            println!("Protocol: {}", http_header.protocol);
        } else {
            println!("Malformed HTTP request line: {}", first_line);
        }
    } else {
        println!("Received an empty HTTP request");
    }



}
