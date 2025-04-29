//! Module dedicated to the [`CreateFile`] I/O-free flow.

use std::path::PathBuf;

use crate::Io;

/// I/O-free flow for creating a file with its contents.
#[derive(Debug)]
pub struct CreateFile {
    input: Option<(PathBuf, Vec<u8>)>,
}

impl CreateFile {
    /// Creates a new flow from the given path and contents.
    pub fn new(path: impl Into<PathBuf>, contents: impl IntoIterator<Item = u8>) -> Self {
        let input = Some((path.into(), contents.into_iter().collect()));
        Self { input }
    }

    /// Makes the flow progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(path) => Io::CreateFile(Err(path)),
                None => Io::UnavailableInput,
            });
        };

        let Io::CreateFile(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match input {
            Ok(()) => Ok(()),
            Err(path) => Err(Io::CreateFile(Err(path))),
        }
    }
}
