pub mod config;

use std::path::PathBuf;
use crate::project::config::ProjectConfig;

#[derive(Eq, PartialEq, Clone)]
pub struct OxideProject {
    pub config: ProjectConfig,
    pub name: String,
    pub directory: PathBuf,
    pub subprojects: Option<Vec<OxideProject>>,
}

impl OxideProject {
    pub fn load(root_path: PathBuf) -> Self {
        let config = ProjectConfig::new(root_path.join("oxide.toml"));

        return OxideProject {
            config: config.clone(),
            name: config.name.clone(),
            directory: root_path.clone(),
            subprojects: OxideProject::load_subprojects(root_path, config),
        }
    }

    pub fn load_subprojects(root_path: PathBuf, config: ProjectConfig) -> Option<Vec<OxideProject>> {
        match config.subprojects.clone() {
            None => None,
            Some(project_dirs) =>
                Some(project_dirs.iter().map(|dir|
                    OxideProject::load(root_path.join(dir))
                ).collect())
        }
    }

    pub fn resync(&mut self) -> bool {
        let new_config = ProjectConfig::new(self.directory.join("oxide.toml"));

        return if new_config == self.config { false }
        else {
            let mut changed = false;
            self.config = new_config.clone();

            let new_name = new_config.name.clone();
            if self.name != new_name {
                self.name = new_name;
                changed = true;
            }

            match self.subprojects.clone() {
                None => { },
                Some(subprojects) => {
                    for mut subproject in subprojects {
                        let subproject_changed = subproject.resync();
                        if subproject_changed { changed = true }
                    }
                }
            };

            changed
        }
    }
}

pub fn find_root_project(current_dir: PathBuf) -> PathBuf {
    let parent = current_dir.parent();
    return match parent {
        None => {
            current_dir
        },
        Some(it) => {
            let parent_dir = it.to_path_buf();
            if is_project(parent_dir.clone()) {
                find_root_project(parent_dir.clone())
            } else {
                current_dir
            }
        }
    }
}

pub fn is_project(current_directory: PathBuf) -> bool {
    return current_directory.join("oxide.toml").exists();
}

pub fn print_project(project: OxideProject, depth: usize) {
    if depth == 0 {
        println!("{} [root]", project.name);
    } else {
        println!("{}{}", "| ".repeat(depth), project.name);
    }

    match project.subprojects {
        None => {} // do nothing
        Some(subprojects) => {
            for subproject in subprojects {
                print_project(subproject, depth + 1);
            }
        }
    }
}