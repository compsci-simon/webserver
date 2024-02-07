use std::net::{TcpListener, TcpStream};
use std::io::{self, Read};

fn main() {
    let listener = TcpListener::bind("localhost:8080").expect("Failed to bind to address");
    
    println!("Listening on port 8080");
    // Accept incoming connections and handle them
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        let peer_addr = stream.peer_addr().expect("Failed to get peer address");
        println!("Accepted connection from: {}", peer_addr);

        if let Err(err) = handle_client(stream) {
            print!("Error handling client: {}", err);
        }
    }
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    // Read from the stream
    let mut buffer: [u8; 1024] = [0; 1024];
    let bytes_read = stream.read(& mut buffer).expect("Failed to read from stream");
    let received_text = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received: {}", received_text);

    Ok(())
}