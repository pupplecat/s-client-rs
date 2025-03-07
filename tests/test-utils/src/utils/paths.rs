use std::path::PathBuf;

use cargo_metadata::MetadataCommand;
use solana_program_test::{find_file, read_file};

pub fn get_workspace_root() -> PathBuf {
    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to fetch workspace metadata");

    metadata.workspace_root.into_std_path_buf()
}

pub fn resolve_path(path: &str) -> PathBuf {
    get_workspace_root().join(path)
}

pub fn test_fixtures_dir() -> PathBuf {
    resolve_path("test-fixtures")
}
