use node_semver::Version;

pub struct NpmPackage {
    pub name: String,
    pub version: Version,
    pub download_url: String,
}