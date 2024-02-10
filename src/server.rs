use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::{
  sync::{ mpsc, Arc, Mutex },
  thread
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();

      println!("Worker {id} got a job; executing.");

      job();
    });

    Worker { id, thread }
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
          workers.push(Worker::new(i, Arc::clone(&receiver)));
            // Create some workers and store them in the vector
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
      let job = Box::new(f);

      self.sender.send(job).unwrap();
    }
}

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
        let pool = ThreadPool::new(2);
        for stream in self.listener.incoming() {
            let stream = stream.expect("Failed to accept connection");
            let peer_addr = stream.peer_addr().expect("Failed to get peer address");
            println!("Accepted connection from: {}", peer_addr);

            pool.execute(|| {
                WebServer::handle_client(stream);
            });
          }
    }

    fn handle_client(mut stream: TcpStream) {
        // Read from the stream
        let mut buffer: [u8; 1024] = [0; 1024];
        let bytes_read = stream.read(& mut buffer).expect("Failed to read from stream");
        let received_text = String::from_utf8_lossy(&buffer[..bytes_read]);

        // println!("Received: {}", received_text);
        if received_text == "" {
          return
        }
        
        let response = WebServer::handle_http_request(&received_text);
        stream.write(response.as_bytes()).expect("Failed to write to stream");
    }

    fn handle_http_request(request: &str) -> String {
        let parts: Vec<&str> = request.split("\r\n").collect();
        let first_line_parts: Vec<&str> = parts[0].split(" ").collect();
        let method = first_line_parts[0];
        let path = first_line_parts.get(1).unwrap();
        let http_version = first_line_parts[2];

        if http_version != "HTTP/1.1" {
            return "HTTP/1.1 505 HTTP Version Not Supported\r\n\r\n".to_string();
        }

        if method != "GET" {
            return "HTTP/1.1 501 Not Implemented\r\n\r\n".to_string();
        }
        WebServer::serve_route(path)
    }

    fn serve_route(route_path: &str) -> String {
      let root_dir = "/Users/simon/Developer/rust-webserver/assets";
      let error_file_path = Path::new(root_dir).join("404.html");
      let error_response = match fs::read_to_string(error_file_path) {
        Ok(content) => format!("HTTP/1.1 404 Page not found\r\n\r\n{content}"),
        Err(_) => {
          "HTTP/1.1 404 Page not found\r\n\r\nPage not found.".to_string()
        }
      };
      let index_response = match fs::read_to_string(Path::new(root_dir).join("index.html")) {
        Ok(content) => {
          let content_len = content.len();
          format!("HTTP/1.1 200 OK\r\nServer: simon-rust/0.1\r\nContent-Length: {content_len}\r\n\r\n{content}")
        },
        Err(err) => {
          println!("Error: {}", err);
          error_response.clone()
        }
      };
      if route_path == "/sleep" {
        thread::sleep(Duration::from_secs(5));
        return "HTTP/1.1 200 OK\r\n\r\nSlept for 3 seconds".to_string();
      }
      if route_path == "/".to_string() || route_path == "/index" {
        return index_response;
      } else {
        return error_response;
      }
    }
}