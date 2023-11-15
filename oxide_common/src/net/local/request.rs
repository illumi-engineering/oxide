pub enum LocalRequest {
    SyncProject { root_dir: String },
    GetProjectGraph { root_dir: String },
}