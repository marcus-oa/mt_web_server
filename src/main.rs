use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

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

    // convert and print byte stream from buffer
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}