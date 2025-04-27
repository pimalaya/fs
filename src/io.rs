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
pub enum Io<'a> {
    CreateDir(&'a mut Result<(), PathBuf>),
    CreateDirs(&'a mut Result<(), HashSet<PathBuf>>),
    CreateFile(&'a mut Result<(), (PathBuf, Vec<u8>)>),
    CreateFiles(&'a mut Result<(), HashMap<PathBuf, Vec<u8>>>),
    ReadDir(&'a mut Result<HashSet<PathBuf>, PathBuf>),
    ReadFile(&'a mut Result<Option<Vec<u8>>, PathBuf>),
    ReadFiles(&'a mut Result<Option<HashMap<PathBuf, Vec<u8>>>, HashSet<PathBuf>>),
    RemoveDir(&'a mut Result<(), PathBuf>),
    RemoveDirs(&'a mut Result<(), HashSet<PathBuf>>),
    RemoveFile(&'a mut Result<(), PathBuf>),
    RemoveFiles(&'a mut Result<(), HashSet<PathBuf>>),
    Rename(&'a mut Result<(), (PathBuf, PathBuf)>),
}
