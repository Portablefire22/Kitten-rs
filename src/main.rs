use std::{fs::File, path::Path, sync::{Arc, Mutex, MutexGuard}, thread::spawn};
mod projects;
use ascii::AsciiString;
use maud::html;
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
}

/*
*   TODO Add a caching system
*/

fn server_thread(server: Arc<Server>, project_handler: Arc<Mutex<ProjectHandler>>) {
    for request in server.incoming_requests() {
        // println!("Request Type: {:?} \nUrl: {:?} \nHeaders: {:?}\n", request.method(), request.url(), request.headers());
        
        let parts = request.url().split("/");
        let parts = parts.collect::<Vec<&str>>();

        // Routing
        match parts[1] {
            "" => {
                let html = construct_page(index(), "").into_string();
                let response = Response::from_string(html);
                let response = response.with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: AsciiString::from_ascii("text/html; charset=utf8").unwrap(),
                });

                let _ = request.respond(response);
            },
            "favicon.ico" => {
                let response = Response::empty(418); // No favicon :3
                let _ = request.respond(response);
            },
            "fonts" => if parts.get(2).is_some() && parts.get(3).is_some() {
                let pat = format!("{}/{}/{}/{}", std::env::current_dir().unwrap().display(), parts[1], parts[2], parts[3]);
                match File::open(pat) {
                    Ok(file) => {
                        let mut response = Response::from_file(file);
                        response = response.with_header(tiny_http::Header {
                            field: "Content-Type".parse().unwrap(),
                            value: "text/css".parse().unwrap()
                        });
                        let _ = request.respond(response);
                    },
                    Err(e) => {
                        println!("{e:?}");
                        let response = Response::empty(404);
                        let _ = request.respond(response);
                    }
                }
            } else {
                let response = Response::empty(404);
                let _ = request.respond(response);
            },
            "css" => if parts.get(2).is_some() {
                let pat = format!("{}/{}/{}", std::env::current_dir().unwrap().display(), parts[1], parts[2]);
                match File::open(pat) {
                    Ok(file) => {
                        let mut response = Response::from_file(file);
                        response = response.with_header(tiny_http::Header {
                            field: "Content-Type".parse().unwrap(),
                            value: "text/css".parse().unwrap()
                        });
                        let _ = request.respond(response);
                    },
                    Err(e) => {
                        println!("{e:?}");
                        let response = Response::empty(404);
                        let _ = request.respond(response);
                    }
                }
            } else {
                let response = Response::empty(404);
                let _ = request.respond(response);
            },
            "assets" => if parts.get(2).is_some() {
                let pat = format!("{}/{}/{}", std::env::current_dir().unwrap().display(), parts[1], parts[2]);
                match File::open(&pat) {
                     Ok(file) => {
                        let mut response = Response::from_file(file);
                        let tmp = Path::new(&pat);
                        // dbg!("{:?}", &tmp.extension());
                        match tmp.extension() {
                            Some(ext) => {
                                response = match ext.to_str().unwrap() {
                                    "png" => response.with_header(tiny_http::Header {
                                        field: "Content-Type".parse().unwrap(),
                                        value: "image/png".parse().unwrap(),
                                    }),
                                    "jpg" | "jpeg" => response.with_header(tiny_http::Header {
                                        field: "Content-Type".parse().unwrap(),
                                        value: "image/jpeg".parse().unwrap(),
                                    }),
                                    "webp" => response.with_header(tiny_http::Header {
                                        field: "Content-Type".parse().unwrap(),
                                        value: "image/webp".parse().unwrap(),
                                    }),
                                    _ => response.with_header(tiny_http::Header {
                                        field: "Content-Type".parse().unwrap(),
                                        value: "image/example".parse().unwrap(),
                                    }),
                                };
                                let _ = request.respond(response);
                            },
                            None => {
                                let response = Response::empty(400);
                                let _ = request.respond(response);
                            } 
                        }
                    },
                    Err(e) => {
                        println!("{e:?}");
                        let response = Response::empty(404);
                        let _ = request.respond(response);
                    }
                }
            } else {
                let response = Response::empty(404);
                let _ = request.respond(response);
            },
            "projects" => if parts.get(2).is_some() {
                let projs = project_handler.lock().unwrap();    
                if projs.projects.iter().any(|p| p.title == parts[2]) {
                    let html = construct_page(render_project(projs, parts[2]), parts[1]).into_string();
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
                let html = construct_page(projects(project_handler.clone()), parts[1]).into_string();
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

/// Inserts given html to a hard-coded template 
fn construct_page(content: maud::Markup, url: &str) -> maud::Markup {
    html! {
        (maud::DOCTYPE)
        body {
            link href="/fonts/Rubik/Rubik-VariableFont_wght.ttf" rel="stylesheet";
            link href="/css/styles.css" rel="stylesheet";
            title {"Kitten.rs"};
            
            (navbar(url))
            (content)
            (footer())
        }
    }
}

fn footer() -> maud::Markup {
    html! {

    }
}

fn navbar(active_page: &str) -> maud::Markup {
    html! {
        div."navigation" {
            div."nav-item" {
                @if active_page == "" {
                    a."active" href="/" {"Home"}
                } @else {
                    a href="/" {"Home"}
                }
            }
            div."nav-item" {
                @if active_page == "projects" {
                    a."active" href="/projects" {"Projects"}
                } @else {
                    a href="/projects" {"Projects"}
                }
            }
            div."nav-item" {
                @if active_page == "about" {
                    a."active" href="/about" {"About"}
                } @else {
                    a href="/about" {"About"}
                }
            }  
        }
        hr;
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
        p { small { (project.formatted_time()) }}
        p { (project.summary) }
        p { (project.html_from_content())}
    }
}

fn projects(project_handler: Arc<Mutex<ProjectHandler>>) -> maud::Markup {
    let project_bind = project_handler.lock().expect("Could not unlock project handler mutex");
    html! {
        @for proj in &project_bind.projects {
            div."content" {
                a."project-link" href={"/projects/"(proj.title)} {
                    div."project-display" {
                        h1 {(proj.title)}
                        small."time" {"UTC: "(proj.formatted_time())}
                        @if proj.image.is_some() {
                            img src={(proj.image.clone().unwrap())};
                        }
                        p {(proj.summary)}
                    }
                }
            }
        }
    }
}

fn serve_404(request: Request) {
    let mut resp = Response::from_string("404");
    resp = resp.with_status_code(404);
    let _ = request.respond(resp);
}
