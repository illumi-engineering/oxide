use std::error::Error;
use std::str::FromStr;
use extism::{host_fn, Plugin, convert::Json};
use oxide_core::NSID;
use super::config::OxideDaemonConfig;

pub mod manager;

host_fn!(get_repositories(cfg: &OxideDaemonConfig, tag: &str) -> Json<Option<Vec<GenericRepositoryConfig>>> {
    Json(cfg.external_repositories.get(tag))
});

pub struct DaemonPlugin<'a> {
    extism_plugin: &'a Plugin,
    id: NSID,
}

pub fn get_id(plugin: &mut Plugin) -> NSID {
    plugin.call::<(), NSID>("id", ()).unwrap()
}

impl DaemonPlugin {
    pub fn wrap(extism_plugin: &mut Plugin) -> Self {
        let id = get_id(extism_plugin);
        Self {
            extism_plugin,
            id,
        }
    }

    pub fn fetch_deps(mut self) -> Result<bool, dyn Error> {
        if self.extism_plugin.function_exists("fetch_deps") {
            let mut res = Ok(true);
            self.extism_plugin.call::<(), ()>("fetch_deps", ()).unwrap_or_else(|e| {
                println!("oxide:daemon:plugin [{}#fetch_deps err] {}", self.id, e);
                res = Err(e);
            });
            res
        } else {
            println!("oxide:daemon:plugin [{}#fetch_deps inf] Call not implemented in plugin, ignoring.", self.id);
            Ok(false)
        }
    }
}