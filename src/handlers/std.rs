//! Module dedicated to the standard, blocking I/O handler.

use std::{
    collections::{HashMap, HashSet},
    fs, io,
    path::PathBuf,
};

use log::debug;

use crate::Io;

/// The standard, blocking I/O handler.
///
/// This handler makes use of standard modules [`std::fs`] and
/// [`std::io`] to manage files and directories.
pub fn handle(input: Io) -> io::Result<Io> {
    match input {
        Io::UnavailableInput => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "input has already been used",
        )),
        Io::UnexpectedInput(io) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unexpected input: {io:?}"),
        )),
        Io::CreateDir(state) => create_dir(state),
        Io::CreateDirs(state) => create_dirs(state),
        Io::CreateFile(state) => create_file(state),
        Io::CreateFiles(state) => create_files(state),
        Io::ReadDir(state) => read_dir(state),
        Io::ReadFile(state) => read_file(state),
        Io::ReadFiles(state) => read_files(state),
        Io::RemoveDir(state) => remove_dir(state),
        Io::RemoveDirs(state) => remove_dirs(state),
        Io::RemoveFile(state) => remove_file(state),
        Io::RemoveFiles(state) => remove_files(state),
        Io::Rename(state) => rename(state),
    }
}

pub fn create_dir(state: Result<(), PathBuf>) -> io::Result<Io> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::create_dir(path)?;

    Ok(Io::CreateDir(Ok(())))
}

pub fn create_dirs(state: Result<(), HashSet<PathBuf>>) -> io::Result<Io> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in paths {
        fs::create_dir(path)?;
    }

    Ok(Io::CreateDirs(Ok(())))
}

pub fn create_file(state: Result<(), (PathBuf, Vec<u8>)>) -> io::Result<Io> {
    let Err((path, contents)) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    fs::write(path, contents)?;

    Ok(Io::CreateFile(Ok(())))
}

pub fn create_files(state: Result<(), HashMap<PathBuf, Vec<u8>>>) -> io::Result<Io> {
    let Err(contents) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    for (path, contents) in contents {
        fs::write(path, contents)?;
    }

    Ok(Io::CreateFiles(Ok(())))
}

pub fn read_dir(state: Result<HashSet<PathBuf>, PathBuf>) -> io::Result<Io> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    let mut paths = HashSet::new();
    let dir = fs::read_dir(path)?;

    for entry in dir {
        match entry {
            Ok(entry) => {
                paths.insert(entry.path());
            }
            Err(err) => {
                debug!("ignore invalid directory entry: {err}");
                continue;
            }
        };
    }

    Ok(Io::ReadDir(Ok(paths)))
}

pub fn read_file(state: Result<Vec<u8>, PathBuf>) -> io::Result<Io> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    let contents = fs::read(path)?;

    Ok(Io::ReadFile(Ok(contents)))
}

pub fn read_files(state: Result<HashMap<PathBuf, Vec<u8>>, HashSet<PathBuf>>) -> io::Result<Io> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    let mut contents = HashMap::new();

    for path in paths {
        let content = fs::read(&path)?;
        contents.insert(path, content);
    }

    Ok(Io::ReadFiles(Ok(contents)))
}

pub fn remove_dir(state: Result<(), PathBuf>) -> io::Result<Io> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::remove_dir_all(path)?;

    Ok(Io::RemoveDir(Ok(())))
}

pub fn remove_dirs(state: Result<(), HashSet<PathBuf>>) -> io::Result<Io> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in paths {
        fs::remove_dir_all(path)?;
    }

    Ok(Io::RemoveDirs(Ok(())))
}

pub fn remove_file(state: Result<(), PathBuf>) -> io::Result<Io> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    fs::remove_file(path)?;

    Ok(Io::RemoveFile(Ok(())))
}

pub fn remove_files(state: Result<(), HashSet<PathBuf>>) -> io::Result<Io> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    for path in paths {
        fs::remove_file(path)?;
    }

    Ok(Io::RemoveFiles(Ok(())))
}

pub fn rename(state: Result<(), (PathBuf, PathBuf)>) -> io::Result<Io> {
    let Err((from, to)) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    fs::rename(from, to)?;

    Ok(Io::Rename(Ok(())))
}
