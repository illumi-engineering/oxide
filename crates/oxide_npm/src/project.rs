use package_json::PackageJsonManager;
use oxide_plugin::{DaemonPluginProjectContext, DaemonPluginProjectScope};

pub struct NpmProjectScope {
    pub(crate) package_json: PackageJsonManager
}

impl DaemonPluginProjectScope for NpmProjectScope {
    fn download_deps(&self, ctx: DaemonPluginProjectContext) -> Result<(), &str> {
        todo!()
    }
}