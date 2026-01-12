use tiny_http::{Server, Response};
use std::fs::File;
use std::thread;

pub fn start_art_server() {
    thread::spawn(|| {
        println!("\nART SERVER:::Starting on http://127.0.0.1:3020");
        let server = Server::http("127.0.0.1:3020").unwrap();
        println!("ART SERVER:::Ready for requests.");
        for request in server.incoming_requests() {
            println!("ART SERVER:::Received request for {}", request.url().to_string());
            let url = request.url().to_string();
            if let Some(stripped) = url.strip_prefix("/artwork/") {
                let path = format!("/tmp/{}", stripped);
                if let Ok(file) = File::open(&path) {
                    println!("ART SERVER:::Serving artwork from {}", path);
                    let _ = request.respond(Response::from_file(file));
                    continue;
                }
            }
            else {
                println!("ART SERVER:::404 Not Found for URL {}", url);
                let _ = request.respond(Response::from_string("Not Found").with_status_code(404));
                println!("ART SERVER:::Responded with 404 Not Found for URL {}", url);
            }
        }
    });
}
