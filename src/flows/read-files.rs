//! Module dedicated to the [`ReadFiles`] I/O-free flow.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::Io;

/// I/O-free flow for reading files contents.
#[derive(Debug)]
pub struct ReadFiles {
    state: Result<Option<HashMap<PathBuf, Vec<u8>>>, HashSet<PathBuf>>,
}

impl ReadFiles {
    /// Reads a new flow from the given files path.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        let state = Err(paths.into_iter().map(Into::into).collect());
        Self { state }
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<HashMap<PathBuf, Vec<u8>>, Io> {
        let Ok(contents) = &mut self.state else {
            return Err(Io::ReadFiles(&mut self.state));
        };

        let Some(contents) = contents.take() else {
            return Err(Io::ReadFiles(&mut self.state));
        };

        Ok(contents)
    }
}
