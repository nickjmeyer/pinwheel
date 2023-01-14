// src/bin/server.rs
use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};
use std::net::Shutdown;
use std::{str, thread, time};
// use std::sync::atomic::AtomicBool;

fn handle_stream(stream: &mut UnixStream) {
    let mut buffer: [u8; 512] = [0; 512];
    println!("Handle stream");
    match stream.read(&mut buffer) {
        Ok(num_bytes) => println!("Read: {:?}", str::from_utf8(&buffer[..num_bytes])),
        Err(_) => println!("Failed to read bytes."),
    }
}

fn start_server(socket_path: &str) {
    let listener = UnixListener::bind(socket_path).expect("Failed to bind socket");
    let addr = listener.local_addr().expect("Couldn't get local address");
    println!("Address: {addr:?}");

    for stream in listener.incoming() {
        println!("For loop!");
        match stream {
            Ok(mut stream) => {
                handle_stream(&mut stream);
                stream.shutdown(Shutdown::Both).expect("Failed to shutdown");
            }
            Err(_) => {
                println!("Stream failed...");
                break;
            }
        }
        break;
    }
}

fn start_client(socket_path: &str) {
    let mut stream = match UnixStream::connect(socket_path) {
        Ok(stream) => stream,
        Err(_) => panic!("Client is unable to connect"),
    };

    match stream.write_all(b"hello world!") {
        Ok(_) => {},
        Err(_) => panic!("Couldn't send message."),
    };
}

fn main() -> std::io::Result<()> {
    let socket_path = "mysocket";

    let server_join_handle = std::thread::spawn(move || { start_server(socket_path) });

    thread::sleep(time::Duration::from_secs(1));

    start_client(socket_path);

    server_join_handle.join().expect("The server thread panicked.");

    Ok(())
}
