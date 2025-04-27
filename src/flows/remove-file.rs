//! Module dedicated to the [`RemoveFile`] I/O-free flow.

use std::path::PathBuf;

use crate::Io;

/// I/O-free flow for removing a file.
#[derive(Debug)]
pub struct RemoveFile {
    state: Result<(), PathBuf>,
}

impl RemoveFile {
    /// Creates a new flow from the given file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let state = Err(path.into());
        Self { state }
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<(), Io> {
        if self.state.is_ok() {
            Ok(())
        } else {
            Err(Io::RemoveFile(&mut self.state))
        }
    }
}
