//! Module dedicated to the [`RemoveFiles`] I/O-free flow.

use std::{collections::HashSet, path::PathBuf};

use crate::Io;

/// I/O-free flow for removing files.
#[derive(Debug)]
pub struct RemoveFiles {
    input: Option<HashSet<PathBuf>>,
}

impl RemoveFiles {
    /// Creates a new flow from the given file paths.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> RemoveFiles {
        let input = Some(paths.into_iter().map(Into::into).collect());
        Self { input }
    }

    /// Makes the flow progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(path) => Io::RemoveFiles(Err(path)),
                None => Io::UnavailableInput,
            });
        };

        let Io::RemoveFiles(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match input {
            Ok(()) => Ok(()),
            Err(path) => Err(Io::RemoveFiles(Err(path))),
        }
    }
}
