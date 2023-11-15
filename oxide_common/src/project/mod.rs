pub mod config;

use std::path::PathBuf;
use crate::project::config::ProjectConfig;

pub struct OxideProject {
    pub config: ProjectConfig,
    pub name: String,
    pub subprojects: Option<Vec<OxideProject>>,
}

impl OxideProject {
    pub fn load(root_path: PathBuf) -> Self {
        let config = ProjectConfig::new(root_path.join("oxide.toml"));

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