use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use mt_web_server::ThreadPool;

fn main() {
    // listen for incoming TCP streams at address 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // hypothetical syntax for creating a ThreadPool
    let pool = ThreadPool::new(4);

    // iterate over received connections on address 127.0.0.1:7878
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream)
        })
    }

    println!("Shutting down.");
}

// read data from TCP stream and print data being sent from
// browser
fn handle_connection(mut stream: TcpStream) {
    // buffer to store up to 1024 bytes of data
    let mut buffer = [0; 1024];

    // read the TCP stream data into the buffer
    stream.read(&mut buffer).unwrap();

    // b to code as bytes
    let get  = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line,filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}