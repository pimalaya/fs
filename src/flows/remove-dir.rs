//! Module dedicated to the [`RemoveDir`] I/O-free flow.

use std::path::PathBuf;

use crate::Io;

/// I/O-free flow for removing a directory.
#[derive(Debug)]
pub struct RemoveDir {
    state: Result<(), PathBuf>,
}

impl RemoveDir {
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
            Err(Io::RemoveDir(&mut self.state))
        }
    }
}
