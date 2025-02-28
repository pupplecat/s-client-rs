use std::path::PathBuf;

use cargo_metadata::MetadataCommand;

pub fn get_workspace_root() -> PathBuf {
    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to fetch workspace metadata");

    metadata.workspace_root.into_std_path_buf()
}

pub fn resolve_path(path: &str) -> PathBuf {
    get_workspace_root().join(path)
}
