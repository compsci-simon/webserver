mod server;
use server::WebServer;

fn main() {
    let webserver = WebServer::new("localhost:8080", "/Users/simon/Developer/rust-webserver/assets".to_string());
    webserver.run();
}
