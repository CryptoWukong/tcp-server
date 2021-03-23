use std::net::{TcpStream};
use std::io::{Write};

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            let msg = b"Hello!";
            stream.write(msg).unwrap(); // send to tcp server
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
