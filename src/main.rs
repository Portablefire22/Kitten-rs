use std::{fmt::Debug, sync::{Arc, Mutex, MutexGuard}, thread::spawn};
mod projects;
use ascii::AsciiString;
use maud::{html, Markup};
use projects::ProjectHandler;
use tiny_http::{Request, Response, Server};

fn main() {
    let server = Arc::new(Server::http("0.0.0.0:8080").unwrap());

    let project_handler = Arc::new(Mutex::new(ProjectHandler::new()));
    project_handler.lock().unwrap().load_projects("./projects/");

    let mut handles = Vec::new();

    for thread_number in 0..std::thread::available_parallelism().expect("Failed to get available paralleism").into() {
        println!("Starting thread: {thread_number}");

        let serv = server.clone();
        let proj = project_handler.clone();
        handles.push(spawn(move || server_thread(serv, proj)));
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


fn server_thread(server: Arc<Server>, project_handler: Arc<Mutex<ProjectHandler>>) {
    for request in server.incoming_requests() {
        println!("Request Type: {:?} \nUrl: {:?} \nHeaders: {:?}\n", request.method(), request.url(), request.headers());
        let parts = request.url().split("/");
        let parts = parts.collect::<Vec<&str>>();
        dbg!(&parts);
        match parts[1] {
            "" => {
                let html = index().into_string();
                let response = Response::from_string(html);
                let response = response.with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: AsciiString::from_ascii("text/html; charset=utf8").unwrap(),
                });

                let _ = request.respond(response);
            },
            "projects" => if parts.get(2).is_some() {
                let projs = project_handler.lock().unwrap();    
                if projs.projects.iter().any(|p| p.title == parts[2]) {
                    let html = render_project(projs, parts[2]).into_string();
                    
                    let response = Response::from_string(html);
                    let response = response.with_header(tiny_http::Header {
                        field: "Content-Type".parse().unwrap(),
                        value: AsciiString::from_ascii("text/html; charset=utf8").unwrap(),
                    });
                    let _ = request.respond(response);
                } else {
                    serve_404(request);
                }
            } else {
                let html = projects(project_handler.clone()).into_string();
                let response = Response::from_string(html);
                let response = response.with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: AsciiString::from_ascii("text/html; charset=utf8").unwrap(),
                });

                let _ = request.respond(response);
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
        a href = "./projects" {"Projects"}
    }
}

fn render_project(project_handler: MutexGuard<ProjectHandler>, project_title: &str) -> maud::Markup {
    let project = project_handler.projects.iter().filter(|p| p.title.eq(project_title)).collect::<Vec<&projects::Project>>();
    let project = project.first().unwrap();
    html! {
        h1 { (project.title) }
        @let project_time: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_timestamp_millis(project.timestamp as i64).unwrap();
        @let localised: chrono::DateTime<chrono::Local> = project_time.into();
        p { small { (localised.format("%Y-%m-%d %H:%M")) }}
        p { (project.summary) }
    }
}

fn projects(project_handler: Arc<Mutex<ProjectHandler>>) -> maud::Markup {
    let project_bind = project_handler.lock().expect("Could not unlock project handler mutex");
    html! {
        @for proj in &project_bind.projects {
            h1 { a href = {"./projects/"(proj.title)} {(proj.title)} }
            @let project_time: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_timestamp_millis(proj.timestamp as i64).unwrap();
            @let localised: chrono::DateTime<chrono::Local> = project_time.into();
            p { small { (localised.format("%Y-%m-%d %H:%M")) }}
            p { (proj.summary) }
        }
    }
}

fn serve_404(request: Request) {
    let mut resp = Response::from_string("404");
    resp = resp.with_status_code(404);
    let _ = request.respond(resp);
}
