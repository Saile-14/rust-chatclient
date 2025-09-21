use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::thread;

fn main()
{
    let mut stream = TcpStream::connect("127.0.0.1:9001").unwrap();
    let stream_clone = stream.try_clone().unwrap();

    thread::spawn(move || {
        let reader = io::BufReader::new(stream_clone);
        for line in reader.lines() {
            println!("> {}", line.unwrap());
        }
    });

    let stdin = io::stdin();
    let mut input = String::new();

        loop {
            input.clear();
            stdin.read_line(&mut input).unwrap();
            if input.trim().is_empty() { continue; }
            stream.write_all(input.trim().as_bytes()).unwrap();
            stream.write_all(b"\n").unwrap();
        }
}