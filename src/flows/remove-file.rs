//! Module dedicated to the [`RemoveFile`] I/O-free flow.

use std::path::PathBuf;

use crate::Io;

/// I/O-free flow for removing a file.
#[derive(Debug)]
pub struct RemoveFile {
    input: Option<PathBuf>,
}

impl RemoveFile {
    /// Creates a new flow from the given file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { input }
    }

    /// Makes the flow progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(path) => Io::RemoveFile(Err(path)),
                None => Io::UnavailableInput,
            });
        };

        let Io::RemoveFile(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match input {
            Ok(()) => Ok(()),
            Err(path) => Err(Io::RemoveFile(Err(path))),
        }
    }
}
