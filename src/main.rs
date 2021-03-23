use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap(); // Listen for TCP connections at the address 127.0.0.1:3333

    for stream in listener.incoming() {     // The incoming method on TcpListener returns an iterator that gives us a sequence of streams
        match stream { // match stream result
            Ok(stream) => {
                handle_client(stream)  // handle client request
            }
            Err(e) => {
                println!("Error: {}", e); // connection failed
            }
        }
    }
    drop(listener); // close the socket server
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    match stream.read(&mut data) { // read bytes from the TcpStream and put them in the buffer.
        Ok(size) => {
            stream.write(&data[0..size]).unwrap(); // sends those bytes directly down the connection.
            let s = String::from_utf8_lossy(&data[0..size]); // convert to string
            println!("From client message: {}", s);
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap(); // shutdown stream
        }
    }
}
