use tiny_http::{Server, Response};
use std::fs::File;
use std::thread;

pub fn start_art_server() {
    thread::spawn(|| {
        println!("Art server: Starting on http://127.0.0.1:3020");
        let server = Server::http("127.0.0.1:3020").unwrap();
        println!("Art server: Listening for requests...");
        for request in server.incoming_requests() {
            println!("Art server: Received request for {}", request.url().to_string());
            let url = request.url().to_string();
            if let Some(stripped) = url.strip_prefix("/artwork/") {
                let path = format!("/tmp/{}", stripped);
                if let Ok(file) = File::open(&path) {
                    println!("Art server: Serving artwork from {}", path);
                    let _ = request.respond(Response::from_file(file));
                    continue;
                }
            }
            else {
                println!("Art server: 404 Not Found for URL {}", url);
                let _ = request.respond(Response::from_string("Not Found").with_status_code(404));
                println!("Art server: Responded with 404 Not Found for URL {}", url);
            }
        }
    });
}
