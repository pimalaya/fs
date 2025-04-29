//! Module dedicated to the [`Rename`] I/O-free flow.

use std::path::PathBuf;

use crate::Io;

/// I/O-free flow for renaming files or directories.
#[derive(Debug)]
pub struct Rename {
    input: Option<(PathBuf, PathBuf)>,
}

impl Rename {
    /// Reads a new flow from the given source and destination paths.
    pub fn new(from: impl Into<PathBuf>, to: impl Into<PathBuf>) -> Self {
        let input = Some((from.into(), to.into()));
        Self { input }
    }

    /// Makes the flow progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(path) => Io::Rename(Err(path)),
                None => Io::UnavailableInput,
            });
        };

        let Io::Rename(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match input {
            Ok(()) => Ok(()),
            Err(path) => Err(Io::Rename(Err(path))),
        }
    }
}
