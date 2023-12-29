use std::collections::HashMap;
use std::path::PathBuf;
use super::OxideProject;

#[derive(Clone)]
pub struct ProjectManager {
    pub projects: HashMap<String, OxideProject>,
}

impl ProjectManager {
    pub fn new() -> Self {
        ProjectManager {
            projects: HashMap::new(),
        }
    }

    pub fn sync(mut self, root: PathBuf) -> bool {
        let project = OxideProject::load(root.clone());

        return if self.projects.contains_key(root.to_str().unwrap()) {
            println!("[oxided] project at {} already registered, resyncing", root.to_str().unwrap());
            self.projects.get_mut(root.to_str().unwrap()).unwrap().resync()
        } else {
            println!("[oxided] registering project at {}", root.to_str().unwrap());
            self.projects.insert(root.to_str().unwrap().parse().unwrap(), project);
            true
        }
    }
}