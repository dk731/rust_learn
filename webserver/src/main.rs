use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

mod HTTP;

fn main() {
    let listener = TcpListener::bind("localhost:3005").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let html_str = fs::read_to_string("./hello.html").unwrap();

    let mut input_buffer = [0; 1024];
    stream.read(&mut input_buffer).unwrap();

    let req = HTTP::Request::new(input_buffer).unwrap();

    // println!("{response}");
}
