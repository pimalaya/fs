//! Module dedicated to the [`CreateDirs`] I/O-free flow.

use std::{collections::HashSet, path::PathBuf};

use crate::Io;

/// I/O-free flow for creating directories.
#[derive(Debug)]
pub struct CreateDirs {
    input: Option<HashSet<PathBuf>>,
}

impl CreateDirs {
    /// Creates a new flow from the given directory paths.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> CreateDirs {
        let input = Some(paths.into_iter().map(Into::into).collect());
        Self { input }
    }

    /// Makes the flow progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(paths) => Io::CreateDirs(Err(paths)),
                None => Io::UnavailableInput,
            });
        };

        let Io::CreateDirs(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match input {
            Ok(()) => Ok(()),
            Err(paths) => Err(Io::CreateDirs(Err(paths))),
        }
    }
}
