pub mod config;

use std::path::PathBuf;

pub struct OxideProject {
    pub config: config::ProjectConfig,
    pub name: String,
    pub subprojects: Option<Vec<OxideProject>>,
}

impl OxideProject {
    pub fn load(root_path: PathBuf) -> Self {
        let config = config::ProjectConfig::new(root_path.join("oxide.toml"));

        return OxideProject {
            config: config.clone(),
            name: config.name.clone(),
            subprojects: match config.subprojects.clone() {
                None => None,
                Some(project_dirs) =>
                    Some(project_dirs.iter().map(|dir|
                        OxideProject::load(root_path.join(dir))
                    ).collect())
            },
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