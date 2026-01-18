//! TODO: Document.

use thiserror::Error;

/// TODO: Document.
#[derive(Debug, Error)]
pub enum HachiyaError {
    #[error("failed to initialize Hachiya; {0}")]
    InitializationError(String),
}
