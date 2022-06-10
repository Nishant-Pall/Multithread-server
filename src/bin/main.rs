use server::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    // listening to tcp requests, by binding the listener to localhost with 7878 port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // create a thread pool of 4 threads ready to pick a job
    let pool = ThreadPool::new(4);

    // loop through connections/streams by iterating over the incoming streams
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // will take a closure and give it to the thread pool for it to be executed
        pool.execute(|| {
            // take in mutable tcp stream as input
            handle_connection(stream)
        })
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

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // convert slices to string including invalid chars
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]))

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // write the response to the stream as bytes
    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}
