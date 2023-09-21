use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    const HOST : &str ="127.0.0.1";
    const PORT : &str ="8080";
    let endpoint : String = HOST.to_owned() + ":" +  PORT;
    let listener = TcpListener::bind(endpoint).unwrap();
    println!("listening on port {}",PORT);
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}