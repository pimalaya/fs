//! Module dedicated to the [`Rename`] I/O-free flow.

use std::path::PathBuf;

use crate::Io;

/// I/O-free flow for renaming files or directories.
#[derive(Debug)]
pub struct Rename {
    state: Result<(), (PathBuf, PathBuf)>,
}

impl Rename {
    /// Reads a new flow from the given source and destination paths.
    pub fn new(from: impl Into<PathBuf>, to: impl Into<PathBuf>) -> Self {
        let state = Err((from.into(), to.into()));
        Self { state }
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<(), Io> {
        if self.state.is_ok() {
            Ok(())
        } else {
            Err(Io::Rename(&mut self.state))
        }
    }
}
