use core::fmt;
use std::{fmt::Debug, fs};
use maud::html;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Project {
    /// Project Title
    pub title: String, 
    /// Unix timestamp for project creation
    pub timestamp: isize,
    /// Short summary of project 
    pub summary: String,
    /// Project's markdown file path
    pub content: String,
    /// Project's icon file path 
    pub image: Option<String>,
}


impl Project {
    /// Creates Project struct from a given file directory
    fn from_file(path: &str) -> Self {
        let contents = fs::read_to_string(path).expect(&format!("Could not find file at '{path}'"));

        let conf: Project = toml::from_str(&contents).expect(&format!("Could not convert '{path}' to project"));
        conf
    }

    /// Returns the project's content markdown file in the form of html
    pub fn html_from_content(&self) -> maud::Markup {
        let contents = fs::read_to_string(&self.content);
        match contents {
            Ok(cont) => {
                maud::PreEscaped(comrak::markdown_to_html(&cont, &comrak::Options::default()))
            },
            Err(_) => {
                html! {
                    div."error" {
                        h1 { "404" }
                        p { "Content file could not be loaded" }
                    }
                }
            }
        }
        
    }

    /// Formats project's unix timestamp to readable UTC
    pub fn formatted_time(&self) -> String {
        let project_time = chrono::DateTime::from_timestamp_millis(self.timestamp as i64);
        match project_time {
            Some(time) => {
                time.format("%Y-%m-%d %H:%M").to_string()
            }, 
            None => "NULL".to_string()
        }
    }
}

/// Contains vector of projects and handles loading/updating stored projects
pub struct ProjectHandler {
    pub projects: Vec<Project>
}

impl ProjectHandler {
    pub fn new() -> Self {
        Self {
            projects: Vec::new()
        }
    }

    /// Loads all project TOMLs from the given directory to the handler's project vector
    pub fn load_projects(&mut self, path: &str) {
        let paths = fs::read_dir(path).expect(&format!("Could not read given directory '{path}'"));
        for p in paths {
            let t = p.unwrap();
            match t.path().extension() {
                Some(ext) => {
                    match ext.to_str().unwrap() {
                        "toml" => {
                            self.projects.push(Project::from_file(t.path().to_str().unwrap()));
                        },
                        _ => (),
                    }
                },
                _ => ()
            }
        }
        self.projects.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    }
}

impl Debug for ProjectHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Project Handler")
            .field("Projects", &self.projects)
            .finish()
    }
}
