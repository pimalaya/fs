//! Module dedicated to the [`CreateDirs`] I/O-free flow.

use std::{collections::HashSet, path::PathBuf};

use crate::Io;

/// I/O-free flow for creating directories.
#[derive(Debug)]
pub struct CreateDirs {
    state: Result<(), HashSet<PathBuf>>,
}

impl CreateDirs {
    /// Creates a new flow from the given directory paths.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> CreateDirs {
        let state = Err(paths.into_iter().map(Into::into).collect());
        Self { state }
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<(), Io> {
        if self.state.is_ok() {
            Ok(())
        } else {
            Err(Io::CreateDirs(&mut self.state))
        }
    }
}
