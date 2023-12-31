use learn_rust::ThreadPool;
use std::{
    fs,
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("localhost:7777").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let content = fs::read_to_string("hello.html").unwrap();
    let resp = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
    stream.write_all(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
