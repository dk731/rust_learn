use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("localhost:3005").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let html_str = fs::read_to_string("./hello.html").unwrap();

    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let content_len = html_str.len();

    let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{html_str}");

    stream.write_all(response.as_bytes()).unwrap();

    // println!("{response}");
}
