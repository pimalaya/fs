//! Module dedicated to the [`ReadDir`] I/O-free flow.

use std::{collections::HashSet, path::PathBuf};

use crate::Io;

/// I/O-free flow for reading directory entries.
#[derive(Debug)]
pub struct ReadDir {
    state: Result<HashSet<PathBuf>, PathBuf>,
}

impl ReadDir {
    /// Reads a new flow from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let state = Err(path.into());
        Self { state }
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<(), Io> {
        if self.state.is_ok() {
            Ok(())
        } else {
            Err(Io::ReadDir(&mut self.state))
        }
    }
}
