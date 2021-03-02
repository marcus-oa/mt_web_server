use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

fn main() {
    // listen for incoming TCP streams at address 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // iterate over received connections on address 127.0.0.1:7878
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

// read data from TCP stream and print data being sent from
// browser
fn handle_connection(mut stream: TcpStream) {
    // buffer to store up to 1024 bytes of data
    let mut buffer = [0; 1024];

    // read the TCP stream data into the buffer
    stream.read(&mut buffer).unwrap();

    // read html file to string
    let contents = fs::read_to_string("hello.html").unwrap();

    // response to connection (Success)
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    // call as_bytes on response to send on TCP stream
    stream.write(response.as_bytes()).unwrap();

    // flush will wait and prevent any program from continuing until
    // all bytes are written to the connection
    stream.flush().unwrap();
}