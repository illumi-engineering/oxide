use crate::package::Package;

trait Repository {
    fn download_package(&self, pkg: Box<dyn Package>) -> Result<(), dyn std::error::Error>;
}