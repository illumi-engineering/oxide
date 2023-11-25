use std::collections::HashMap;
use oxide_plugin::{DaemonPlugin, DaemonPluginProjectContext, DaemonPluginProjectScope};
use oxide_project::OxideProject;

pub struct DaemonPluginManager<'a> {
    plugins: HashMap<String, &'a dyn DaemonPlugin<&'a dyn DaemonPluginProjectScope>>,
}

impl DaemonPluginManager<'_> {
    fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    fn download_dependencies(self, project: &OxideProject) -> Result<(), &str> { // todo: error type
        let ctx = DaemonPluginProjectContext {
            project: Box::new((*project).into()),
        };

        let mut error = false;
        let mut res: Result<(), &str> = Ok(());

        self.plugins.iter().for_each(|(id, &plugin)| {
            if res.is_err() { return } // abort on error
            let download_res = (*plugin).use_project(project).download_deps(ctx);

            match download_res {
                Ok(_) => {}
                Err(msg) => {
                    eprintln!("[oxide:plugin] err! in plugin '{}': {}", id, msg);
                    error = true;
                    res = Err(msg);
                }
            }
        });

        res
    }
}