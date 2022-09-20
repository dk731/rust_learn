use chrono;
use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

mod HTTP;
mod ThreadsPool;

fn main() {
    let listener = TcpListener::bind("localhost:3005").unwrap();
    let mut threads_pool = ThreadsPool::Pool::new(5);

    for stream in listener.incoming() {
        threads_pool.excecute(move || {
            let stream = stream.unwrap();
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let html_str = fs::read_to_string("./hello.html").unwrap();

    let mut input_buffer = [0; 1024];
    stream.read(&mut input_buffer).unwrap();

    let req = HTTP::Request::new(&input_buffer).unwrap();
    let mut res = HTTP::Response::new();

    res.headers
        .insert("Date".to_string(), chrono::offset::Utc::now().to_string());

    res.set_body(&html_str);

    stream.write_all(res.result().as_bytes());
}
