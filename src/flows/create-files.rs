//! Module dedicated to the [`CreateFiles`] I/O-free flow.

use std::{collections::HashMap, path::PathBuf};

use crate::Io;

/// I/O-free flow for creating multiple files with their contents.
#[derive(Debug)]
pub struct CreateFiles {
    input: Option<HashMap<PathBuf, Vec<u8>>>,
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
        let input = Some(contents);
        Self { input }
    }

    /// Makes the flow progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        let Some(input) = input else {
            return Err(match self.input.take() {
                Some(path) => Io::CreateFiles(Err(path)),
                None => Io::UnavailableInput,
            });
        };

        let Io::CreateFiles(input) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match input {
            Ok(()) => Ok(()),
            Err(path) => Err(Io::CreateFiles(Err(path))),
        }
    }
}
