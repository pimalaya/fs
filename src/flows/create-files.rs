//! Module dedicated to the [`CreateFiles`] I/O-free flow.

use std::{collections::HashMap, path::PathBuf};

use crate::Io;

/// I/O-free flow for creating multiple files with their contents.
#[derive(Debug)]
pub struct CreateFiles {
    state: Result<(), HashMap<PathBuf, Vec<u8>>>,
}

impl CreateFiles {
    /// Creates a new flow from the given contents.
    pub fn new(
        contents: impl IntoIterator<Item = (impl Into<PathBuf>, impl IntoIterator<Item = u8>)>,
    ) -> Self {
        let contents = contents
            .into_iter()
            .map(|(path, contents)| (path.into(), contents.into_iter().collect()))
            .collect();
        let state = Err(contents);
        Self { state }
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<(), Io> {
        if self.state.is_ok() {
            Ok(())
        } else {
            Err(Io::CreateFiles(&mut self.state))
        }
    }
}
