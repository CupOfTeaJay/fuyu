//! TODO: Document.

mod applicator;
mod exceptions;
mod plugin;
mod registrar;
mod repository;
mod services;

pub use crate::plugin::HachiyaPlugin;
pub use crate::registrar::Registrar;
pub use crate::repository::{BuildTarget, ModRepository};
pub use crate::services::{RepositoryRequest, RepositoryService};
