use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // listening to tcp requests, by binding the listener to localhost with 7878 port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // loop through connections/streams by iterating over the incoming streams
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // take in mutable tcp stream as input
        handle_connection(stream)
    }
}

// read data from stream
fn handle_connection(mut stream: TcpStream) {
    // create buffer for data that is read
    // 1204 bytes long
    let mut buffer = [0; 1024];

    // populate buffer from the data in the stream
    // read mutates internal state so we need mutable stream param
    stream.read(&mut buffer).unwrap();

    // convert slices to string including invalid chars
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]))

    let contents = fs::read_to_string("index.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    // write the response to the stream as bytes
    stream.write(response.as_bytes()).unwrap();

    // flush the stream
    stream.flush().unwrap();
}
