use std::{fs::{self, File}, io::Read, path::Path, sync::{Arc, Mutex, MutexGuard}, thread::spawn};
mod projects;
use ascii::AsciiString;
use comrak::plugins::syntect::SyntectAdapterBuilder;
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
        println!("Request Type: {:?} \nUrl: {:?} \nHeaders: {:?}\n", request.method(), request.url(), request.headers());
        
        let parts = request.url().split("/");
        let mut parts = parts.collect::<Vec<&str>>();

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
                let temp = parts[2].replace("%20", " ");
                parts[2] = &temp;
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
                    serve_error(request, 404);
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
            "about" => {
                let html = construct_page(about(), parts[1]).into_string();
                let response = Response::from_string(html);
                let response = response.with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: AsciiString::from_ascii("text/html; charset=utf8").unwrap(),
                });

                let _ = request.respond(response);
            },
            ".well-known" => if parts.get(2).is_some() {
               if parts[2] == "discord" {
                    let mut file = File::open("assets/discord").unwrap();
                    let mut cont = String::new();
                    file.read_to_string(&mut cont).unwrap();
                    let response = Response::from_string(cont);
                    let response = response.with_header(tiny_http::Header {
                        field: "Content-Type".parse().unwrap(),
                        value: AsciiString::from_ascii("text/html; charset=utf8").unwrap(),
                    });
                    let _ = request.respond(response);
                } 
            } else {
                serve_error(request, 404);
            },
            _ => {
                serve_error(request, 404);
            }
        }
    }
}

fn about() -> maud::Markup {
    let contents = fs::read_to_string("projects/about.md");
    let mut plugins = comrak::Plugins::default();
    let builder = SyntectAdapterBuilder::new().theme("base16-ocean.dark");
    let adapter = builder.build();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);
    match contents {
        Ok(cont) => {
            html! {
                div."content-container" {
                    h1 {"About"}
                    div."project-text"{
                        (maud::PreEscaped(comrak::markdown_to_html_with_plugins(&cont,
                            &comrak::Options::default(),
                            &plugins)))
                    }    
                }
            }
        },
        Err(e) => error(404),
    }
}

/// Inserts given html to a hard-coded template 
fn construct_page(content: maud::Markup, url: &str) -> maud::Markup {
    html! {
        (maud::DOCTYPE)
        body {
            link rel="preload stylesheet" href="/css/styles.css" as="style" type="text/css" crossorigin="anonymous";
            title {"Kitten.rs"};
            (navbar(url))
            (content)
            (footer())
        }
    }
}

fn footer() -> maud::Markup {
    html! {
        div."footer" {
            hr;
            div."footer-links" {
                a href="https://github.com/Portablefire22" {"Github"}
                a href="https://www.youtube.com/@YuumiOTP" {"Youtube"}
                a href="https://github.com/Portablefire22/Kitten-rs" {"Source"}
            }
        }
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
       
        h1."c" { "Kitten.rs" }
        h2."c" { "Under Construction" }
    }
}

fn render_project(project_handler: MutexGuard<ProjectHandler>, project_title: &str) -> maud::Markup {
    let project = project_handler.projects.iter().filter(|p| p.title.eq(project_title)).collect::<Vec<&projects::Project>>();
    let project = project.first().unwrap();
    html! {

        div."content-container" {
            h1 {(project.title)}
            small."time" {"UTC: "(project.formatted_time())}
            div."project-text"{
                @if project.image.is_some() {
                    img."icon" src={(project.image.clone().unwrap())};
                }
                p {(project.html_from_content())}
            }
        }
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

fn error(code: u16) -> maud::Markup {
    html!{div."error" {
            h1 { (code) }
        }
    }
}

fn serve_error(request: Request, code: u16) {
    let html = construct_page(error(code), "error").into_string();
    let mut resp = Response::from_string(html);
    resp = resp.with_status_code(404);
    resp = resp.with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: AsciiString::from_ascii("text/html; charset=utf8").unwrap(),
                });


    let _ = request.respond(resp);
}
