use web_server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buf = [0; 1024];

  stream.read(&mut buf).unwrap();


  let get_idx = b"GET / HTTP/1.1\r\n";
  let get_sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status, content) = if buf.starts_with(get_idx) {
    ("200 OK", get_page("./index.html"))
  } else if buf.starts_with(get_sleep) {
    thread::sleep(Duration::from_secs(5));

    ("200 OK", get_page("./index.html"))
  } else {
    ("404 NOT FOUND", get_page("./404.html"))
  };

  let res = format!(
    "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
    status,
    content.len(),
    content
  );

  stream.write(res.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn get_page(path: &str) -> String {
  fs::read_to_string(path).unwrap()
}
