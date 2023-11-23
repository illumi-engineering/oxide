use oxide_core::Package;
use oxide_core::Repository;
use oxide_core::NSID;

trait DaemonPlugin<T : Package> {
    const IDENTIFIER: NSID;

    fn register_repository(&self, name: String, repo: Box<dyn Repository<T>>);
}