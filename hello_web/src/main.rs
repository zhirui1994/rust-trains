use std::io::prelude::*;
use std::net::*;

#[warn(unused_variables)]
fn main() {
    let listener = TcpListener::bind("localhost:7788").expect("TCP 监听绑定失败！");

    for stream in listener.incoming() {
        let stream = stream.expect("监听流读取失败！");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
