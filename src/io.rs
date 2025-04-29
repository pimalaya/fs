use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

/// The fs I/O request enum, emitted by flows and processed by
/// handlers.
///
/// This enum represents all the possible I/O requests that a file
/// system flow can emit. I/O handlers should be able to handle all
/// variants.
#[derive(Debug)]
pub enum Io {
    UnavailableInput,
    UnexpectedInput(Box<Io>),

    CreateDir(Result<(), PathBuf>),
    CreateDirs(Result<(), HashSet<PathBuf>>),
    CreateFile(Result<(), (PathBuf, Vec<u8>)>),
    CreateFiles(Result<(), HashMap<PathBuf, Vec<u8>>>),
    ReadDir(Result<HashSet<PathBuf>, PathBuf>),
    ReadFile(Result<Vec<u8>, PathBuf>),
    ReadFiles(Result<HashMap<PathBuf, Vec<u8>>, HashSet<PathBuf>>),
    RemoveDir(Result<(), PathBuf>),
    RemoveDirs(Result<(), HashSet<PathBuf>>),
    RemoveFile(Result<(), PathBuf>),
    RemoveFiles(Result<(), HashSet<PathBuf>>),
    Rename(Result<(), (PathBuf, PathBuf)>),
}
