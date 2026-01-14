///! TODO: Document.
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Deserialization target for a Cargo manifest.
#[derive(Debug, Deserialize)]
pub struct Manifest {
    /// TODO: Document.
    workspace: Workspace,
}

/// Deserialization target for a Cargo manifest's workspace.
#[derive(Debug, Deserialize)]
struct Workspace {
    /// Dependencies shared by all workspace [members].
    #[serde(default)]
    dependencies: HashMap<String, toml::Value>,

    /// Packages that belong to the workspace.
    members: Vec<String>,

    /// The feature resolution algorithm to use.
    resolver: String,
}

/// TODO: Document.
pub fn get_manifest<P: AsRef<Path>>(path: P) -> Manifest {
    toml::from_str(
        &fs::read_to_string(path.as_ref().join("Cargo.toml")).expect("failed to read manifest"),
    )
    .expect("failed to deserialize Cargo.toml")
}
