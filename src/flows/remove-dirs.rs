//! Module dedicated to the [`RemoveDirs`] I/O-free flow.

use std::{collections::HashSet, path::PathBuf};

use crate::Io;

/// I/O-free flow for removing directories.
#[derive(Debug)]
pub struct RemoveDirs {
    state: Result<(), HashSet<PathBuf>>,
}

impl RemoveDirs {
    /// Creates a new flow from the given directory paths.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> RemoveDirs {
        let state = Err(paths.into_iter().map(Into::into).collect());
        Self { state }
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<(), Io> {
        if self.state.is_ok() {
            Ok(())
        } else {
            Err(Io::RemoveDirs(&mut self.state))
        }
    }
}
