extern crate multithreaded_server_test;
use multithreaded_server_test::ThreadPool;

use std::io::prelude::*;
use std::fs::File;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;


fn main() {
    let address: &str = "127.0.0.1:7878";
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream: TcpStream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 512] = [0; 512]; 
    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let sleep: &[u8; 21] = b"GET /sleep HTTP/1.1\r\n";
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "html/test.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "html/test.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND \r\n\r\n", "html/404.html")       
    };
    let mut file: File = File::open(filename).unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    let response: String = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
