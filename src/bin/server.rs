use std::arch::x86_64::_CMP_FALSE_OQ;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener,TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, vec};

fn handle_client(stream: TcpStream, peers: Arc<Mutex<Vec<std::net::TcpStream>>>)
{
    let mut writer = stream.try_clone().unwrap();
    let addr = stream.peer_addr().unwrap();
    let reader = BufReader::new(stream.try_clone().unwrap());

    peers.lock().unwrap().push(writer.try_clone().unwrap());

    let peers_clone = peers.clone();
    thread::spawn(move || {
        for line in reader.lines() {
            let msg = format!("{}: {}", addr, line.unwrap());

            let mut peers = peers_clone.lock().unwrap();
            peers.retain_mut(|peer| {
                if writeln!(peer, "{}", msg).is_err() {
                    false
                } else {
                    true
                }
            });
        }
    });

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    println!("Chat server up on 127.0.0.1:9001");

    let peers: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming()
    {
        let stream = stream.unwrap();
        let peers_clone = peers.clone();

        thread::spawn(move || {
            handle_client(stream, peers_clone);
        });
    }
}
