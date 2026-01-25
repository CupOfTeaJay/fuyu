//! TODO: Document.

use bevy::prelude::*;

use crate::repository::BuildTarget;

/// TODO: Document.
#[derive(Clone, Debug, Message)]
pub struct RepositoryRequest(pub RepositoryService);

/// TODO: Document.
#[derive(Clone, Debug)]
pub enum RepositoryService {
    BuildMod(String, BuildTarget),
    BuildRepository(BuildTarget),
    LoadMod(String, BuildTarget),
    LoadRepository(BuildTarget),
    UnloadMod(String),
    UnloadRepository,
}
