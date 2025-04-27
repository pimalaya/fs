//! Module dedicated to the [`CreateDir`] I/O-free flow.

use std::path::PathBuf;

use crate::Io;

/// I/O-free flow for creating a directory.
#[derive(Debug)]
pub struct CreateDir {
    state: Result<(), PathBuf>,
}

impl CreateDir {
    /// Creates a new flow from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let state = Err(path.into());
        Self { state }
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<(), Io> {
        if self.state.is_ok() {
            Ok(())
        } else {
            Err(Io::CreateDir(&mut self.state))
        }
    }
}
