use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::path::Path;

pub struct WebServer {
    listener: TcpListener,
    root_dir: String
}

impl WebServer {
    pub fn new(address: &str, root_dir: String) -> WebServer {
        let listener = TcpListener::bind(address).expect("Failed to bind to address");
        WebServer {
            listener,
            root_dir
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
        if received_text == "" {
          return Ok(())
        }
        
        let response = self.handle_http_request(&received_text);
        stream.write(response.as_bytes()).expect("Failed to write to stream");
        Ok(())
    }

    fn handle_http_request(&self, request: &str) -> String {
        let parts: Vec<&str> = request.split("\r\n").collect();
        let first_line_parts: Vec<&str> = parts[0].split(" ").collect();
        let method = first_line_parts[0];
        let path = first_line_parts.get(1).unwrap();
        let http_version = first_line_parts[2];
        println!("Method: {}, Path: {}, HTTP Version: {}", method, path, http_version);

        if method != "GET" {
            return "HTTP/1.1 501 Not Implemented\r\n\r\n".to_string();
        }
        self.serve_route(path)
    }

    fn serve_route(&self, route_path: &str) -> String {
      let error_file_path = Path::new(&self.root_dir).join("404.html");
      let error_response = match fs::read_to_string(error_file_path) {
        Ok(content) => format!("HTTP/1.1 404 Page not found\r\n\r\n{content}"),
        Err(_) => {
          "HTTP/1.1 404 Page not found\r\n\r\nPage not found.".to_string()
        }
      };
      let index_response = match fs::read_to_string(Path::new(&self.root_dir).join("index.html")) {
        Ok(content) => {
          let content_len = content.len();
          format!("HTTP/1.1 200 OK\r\nServer: simon-rust/0.1\r\nContent-Length: {content_len}\r\n\r\n{content}")
        },
        Err(err) => {
          println!("Error: {}", err);
          error_response.clone()
        }
      };
      let favicon_response = match fs::read_to_string(Path::new(&self.root_dir).join("favicon.png")) {
        Ok(content) => format!("HTTP/1.1 200 OK\r\n\r\n{content}"),
        Err(err) => {
          println!("Error: {}", err);
          error_response.clone()
        }
      };
      if route_path == "/".to_string() || route_path == "/index" {
        return index_response;
      // } else if route_path == "/favicon.png" {
      //   return favicon_response;
      } else {
        return error_response;
      }
    }
}