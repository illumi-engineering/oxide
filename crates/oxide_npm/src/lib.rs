mod config;
mod project;
mod package;

use std::str::FromStr;
use oxide_core::{GenericRepositoryConfig, NSID};
use extism_pdk::*;

#[host_fn]
extern "ExtismHost" {
    fn get_repositories(key: &str) -> Json<Vec<GenericRepositoryConfig>>;
}

static NPM_IDENTIFIER: &str = "oxide:npm";

#[plugin_fn]
pub fn id() -> FnResult<NSID> {
    Ok(NSID::from_str(NPM_IDENTIFIER).unwrap())
}

#[plugin_fn]
pub fn fetch_deps() -> FnResult<()> {
    let repos = unsafe { get_repositories("npm").unwrap().0 };
    for repo in repos {
        let GenericRepositoryConfig { url, label } = repo;
        // todo: download deps
    }
    Ok(())
}
