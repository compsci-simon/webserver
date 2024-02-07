use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub struct WebServer {
    listener: TcpListener,
}

impl WebServer {
    pub fn new(address: &str) -> WebServer {
        let listener = TcpListener::bind(address).expect("Failed to bind to address");
        WebServer {
            listener,
        }
    }

    pub fn run(&self) {
        println!("Listening on port 8080");
        for stream in self.listener.incoming() {
            let stream = stream.expect("Failed to accept connection");
            let peer_addr = stream.peer_addr().expect("Failed to get peer address");
            println!("Accepted connection from: {}", peer_addr);

            if let Err(err) = self.handle_client(stream) {
                print!("Error handling client: {}", err);
            }
        }
    }

    fn handle_client(&self, mut stream: TcpStream) -> std::io::Result<()> {
        // Read from the stream
        let mut buffer: [u8; 1024] = [0; 1024];
        let bytes_read = stream.read(& mut buffer).expect("Failed to read from stream");
        let received_text = String::from_utf8_lossy(&buffer[..bytes_read]);

        // println!("Received: {}", received_text);
        
        let response = self.handle_http_request(&received_text);
        stream.write(response.as_bytes()).expect("Failed to write to stream");
        Ok(())
    }

    fn handle_http_request(&self, request: &str) -> String {
        let parts: Vec<&str> = request.split("\r\n").collect();
        let first_line_parts: Vec<&str> = parts[0].split(" ").collect();
        let method = first_line_parts[0];
        let path = first_line_parts[1];
        let http_version = first_line_parts[2];
        println!("Method: {}, Path: {}, HTTP Version: {}", method, path, http_version);

        let response = format!("HTTP/1.1 200 OK\r\n\r\nHello world!");
        response
    }
}