use oxide_project::OxideProject;

pub struct DaemonPluginProjectContext {
    pub project: Box<OxideProject>
}

pub trait DaemonPluginProjectScope {
    fn download_deps(&self, ctx: DaemonPluginProjectContext) -> Result<(), &str>;
}