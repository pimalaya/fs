//! Module dedicated to the [`RemoveFiles`] I/O-free flow.

use std::{collections::HashSet, path::PathBuf};

use crate::Io;

/// I/O-free flow for removing files.
#[derive(Debug)]
pub struct RemoveFiles {
    state: Result<(), HashSet<PathBuf>>,
}

impl RemoveFiles {
    /// Creates a new flow from the given file paths.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> RemoveFiles {
        let state = Err(paths.into_iter().map(Into::into).collect());
        Self { state }
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<(), Io> {
        if self.state.is_ok() {
            Ok(())
        } else {
            Err(Io::RemoveFiles(&mut self.state))
        }
    }
}
