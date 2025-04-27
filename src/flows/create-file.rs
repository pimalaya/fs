//! Module dedicated to the [`CreateFile`] I/O-free flow.

use std::path::PathBuf;

use crate::Io;

/// I/O-free flow for creating a file with its contents.
#[derive(Debug)]
pub struct CreateFile {
    state: Result<(), (PathBuf, Vec<u8>)>,
}

impl CreateFile {
    /// Creates a new flow from the given path and contents.
    pub fn new(path: impl Into<PathBuf>, contents: impl IntoIterator<Item = u8>) -> Self {
        let state = Err((path.into(), contents.into_iter().collect()));
        Self { state }
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<(), Io> {
        if self.state.is_ok() {
            Ok(())
        } else {
            Err(Io::CreateFile(&mut self.state))
        }
    }
}
