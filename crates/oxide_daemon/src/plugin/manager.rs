use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf};
use oxide_core::NSID;
use extism::*;
use crate::config::OxideDaemonConfig;
use crate::plugin::{DaemonPlugin, get_repositories};

pub struct PluginManager<'a> {
    plugin_dir: PathBuf,
    plugins: HashMap<NSID, &'a mut DaemonPlugin<'a>>,
    config: UserData<OxideDaemonConfig>,
}

impl PluginManager {
    pub fn new(plugin_dir: PathBuf, config: &OxideDaemonConfig) -> Self {
        Self {
            plugin_dir,
            plugins: HashMap::new(),
            config: UserData::new(config.into()),
        }
    }

    /// Load plugin files from [plugin_dir]
    pub fn load_plugins(&mut self) -> Result<(), ()> {
        let plugin_paths = fs::read_dir(self.plugin_dir.clone()).unwrap();

        // let mut watcher = notify::recommended_watcher(|res: notify::Result<notify::Event>| {
        //     match res {
        //         Ok(event) => {
        //             match event.kind {
        //                 EventKind::Modify(modify_kind) => {
        //                     match modify_kind {
        //                         ModifyKind::Data(data_change) => {
        //                             if data_change == DataChange::Content {
        //                                 for path in event.paths {
        //                                     if path.metadata().unwrap().is_dir() { continue; } else if path.file_name().unwrap().into_os_string().into_string().unwrap().ends_with(".wasm") {
        //                                         let file = Wasm::file(path);
        //                                         let manifest = Manifest::new([file]);
        //                                         let mut extism_plugin = Plugin::new(&manifest, [], true)?;
        //
        //                                         if !extism_plugin.function_exists("id") {
        //                                             println!("oxide:plugin [warn] Plugin at '{}' does not implement the required `id` call, refusing to reload.", path.file_name().unwrap().into_os_string().into_string().unwrap());
        //                                             continue;
        //                                         }
        //
        //                                         let id = get_id(&mut extism_plugin);
        //                                         if self.plugins.contains_key(&id) {
        //                                             self.plugins.get_mut(&id).unwrap().extism_plugin.update_with_manifest(manifest)
        //                                         }
        //                                     }
        //                                 }
        //                             }
        //                         },
        //                         _ => {}
        //                     }
        //                 },
        //                 _ => {}
        //             }
        //         },
        //         Err(e) => println!("watch error: {:?}", e),
        //     }
        // })?;

        for plugin_path in plugin_paths {
            if plugin_path.unwrap().metadata().unwrap().is_dir() { continue; }
            else if plugin_path.unwrap()
                // Take file name and convert it into String since Rust needs 50
                // different string types
                .file_name().into_string().unwrap()
                .ends_with(".wasm") { // Is it a wasm module?
                // if so, load it and get the id
                let file = Wasm::file(plugin_path);
                let manifest = Manifest::new([file]);
                let mut extism_plugin = PluginBuilder::new(&manifest)
                    .with_wasi(true)
                    .with_function(
                        "get_repositories",
                        [PTR],
                        [PTR],
                        self.config.clone(),
                        get_repositories,
                    )
                    .build().unwrap();

                if !extism_plugin.function_exists("id") {
                    println!("oxide:plugin [warn] Plugin at '{}' does not implement the required `id` call, refusing to load.", plugin_path.unwrap().file_name().into_string().unwrap());
                    continue;
                }

                let mut plugin = DaemonPlugin::wrap(&mut extism_plugin);
                self.plugins.insert(plugin.id, &mut plugin);

                // Add a path to be watched. All files and directories at that path and
                // below will be monitored for changes.
                // watcher.watch(plugin_path.unwrap().path().as_path(), RecursiveMode::NonRecursive)?;
            }
        }

        Ok(())
    }

    pub fn do_each<F : FnMut(DaemonPlugin)>(&mut self, f: F) {
        self.plugins.values_mut().for_each(f);
    }
}