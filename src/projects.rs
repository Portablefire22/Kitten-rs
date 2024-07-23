use core::fmt;
use std::{fmt::Debug, fs, path::Path};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Project {
    pub title: String, 
    pub timestamp: isize,
    pub summary: String,
    pub content: String, // Path to markdown file 
    pub image: Option<String>, // Path to project icon
}


impl Project {
    fn from_file(path: &str) -> Self {
        let contents = fs::read_to_string(path).expect("Could not find file at '{path}'");

        let conf: Project = toml::from_str(&contents).expect("Could not convert '{path}' to project");
        conf
    }
}


pub struct ProjectHandler {
    pub projects: Vec<Project>
}

impl ProjectHandler {
    pub fn new() -> Self {
        Self {
            projects: Vec::new()
        }
    }

    pub fn load_projects(&mut self, path: &str) {
        let paths = fs::read_dir(path).expect("Could not read given directory '{path}'");
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
