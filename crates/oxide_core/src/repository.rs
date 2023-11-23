use crate::package::Package;

pub trait Repository<T : Package> {
    fn download_package(&self) -> T;
}