mod project;

pub use {project::*};

use oxide_core::NSID;
use oxide_project::OxideProject;

pub trait DaemonPlugin<S : DaemonPluginProjectScope + ?Sized> {
    fn id() -> NSID;

    fn use_project(&self, project: &OxideProject) -> S;
}

pub trait ClientPlugin {
    fn id() -> NSID;
}