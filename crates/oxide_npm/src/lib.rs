mod config;
mod project;
mod package;

use std::str::FromStr;
use package_json::PackageJsonManager;
use oxide_core::{NSID};
use oxide_plugin::{ClientPlugin, DaemonPlugin};
use oxide_project::OxideProject;
use crate::project::NpmProjectScope;

const NPM_IDENTIFIER: NSID = NSID::from_str("npm").unwrap();

pub struct OxideNpmDaemonPlugin {}

impl DaemonPlugin<NpmProjectScope> for OxideNpmDaemonPlugin {
    fn id(self) -> NSID { NPM_IDENTIFIER }

    fn use_project(&self, project: &OxideProject) -> NpmProjectScope {
        NpmProjectScope {
            package_json: PackageJsonManager::with_file_path(project.directory.clone().join("package.json"))
        }
    }
}

pub struct OxideNpmClientPlugin {}

impl ClientPlugin for OxideNpmClientPlugin {
    fn id(self) -> NSID { NPM_IDENTIFIER }
}