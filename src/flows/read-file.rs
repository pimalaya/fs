//! Module dedicated to the [`ReadFile`] I/O-free flow.

use std::path::PathBuf;

use crate::Io;

/// I/O-free flow for reading file contents.
#[derive(Debug)]
pub struct ReadFile {
    state: Result<Option<Vec<u8>>, PathBuf>,
}

impl ReadFile {
    /// Reads a new flow from the given file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let state = Err(path.into());
        Self { state }
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<Vec<u8>, Io> {
        let Ok(contents) = &mut self.state else {
            return Err(Io::ReadFile(&mut self.state));
        };

        let Some(contents) = contents.take() else {
            return Err(Io::ReadFile(&mut self.state));
        };

        Ok(contents)
    }
}
