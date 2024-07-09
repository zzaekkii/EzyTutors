use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let connection_listner = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    for stream in connection_listner.incoming() {
        let mut _stream = stream.unwrap();
        println!("Connection established");
        let mut buffer = [0; 1024];
        _stream.read(&mut buffer).unwrap();
        _stream.write(&mut buffer).unwrap();
    }
}
