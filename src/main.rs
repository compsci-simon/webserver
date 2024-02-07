mod server;
use server::WebServer;

fn main() {
    let webserver = WebServer::new("localhost:8080");
    webserver.run();
}
