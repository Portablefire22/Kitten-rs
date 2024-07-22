use std::{sync::Arc, thread::spawn};

use ascii::AsciiString;
use maud::html;
use tiny_http::{Request, Response, Server};

fn main() {
    let server = Arc::new(Server::http("0.0.0.0:8080").unwrap());

    let mut handles = Vec::new();

    for thread_number in 0..std::thread::available_parallelism().expect("Failed to get available paralleism").into() {
        println!("Starting thread: {thread_number}");

        let serv = server.clone();
        handles.push(spawn(move || server_thread(serv)));
    }

    for handle in handles {
        match handle.join() {
            Err(e) => {
                println!("{e:?}");
            },
            Ok(_) => (),
        }
    }

    // for request in server.incoming_requests() {
    //     println!("{:?} | {:?} | {:?}", request.method(), request.url(), request.headers());
    //     
    //     let response = Response::from_string("Hello World!");
    //     request.respond(response);
    // }
}


fn server_thread(server: Arc<Server>) {
    for request in server.incoming_requests() {
        println!("Request Type: {:?} \nUrl: {:?} \nHeaders: {:?}", request.method(), request.url(), request.headers());
       
        match request.url() {
            "" | "/" => {
                let html = index().into_string();
                let response = Response::from_string(html);
                let response = response.with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: AsciiString::from_ascii("text/html; charset=utf8").unwrap(),
                });

                request.respond(response);
            },
            _ => {
                serve_404(request);
            }
        }

    }
}

fn index() -> maud::Markup {
    html! { 
        h1 { "Kitten.rs" }
        p.intro {
            "Testing hyper links '"
        a href = "https://github.com/portablefire22" {"Github"}
        "'"
        }
    }
}

fn serve_404(request: Request) {
    let mut resp = Response::from_string("404");
    resp = resp.with_status_code(404);
    request.respond(resp);
}
