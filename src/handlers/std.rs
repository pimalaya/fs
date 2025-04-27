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
pub fn handle(io: Io) -> io::Result<()> {
    match io {
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

pub fn create_dir(state: &mut Result<(), PathBuf>) -> io::Result<()> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::create_dir(path)?;

    *state = Ok(());
    Ok(())
}

pub fn create_dirs(state: &mut Result<(), HashSet<PathBuf>>) -> io::Result<()> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in &*paths {
        fs::create_dir(path)?;
    }

    *state = Ok(());
    Ok(())
}

pub fn create_file(state: &mut Result<(), (PathBuf, Vec<u8>)>) -> io::Result<()> {
    let Err((path, contents)) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    fs::write(path, contents)?;

    *state = Ok(());
    Ok(())
}

pub fn create_files(state: &mut Result<(), HashMap<PathBuf, Vec<u8>>>) -> io::Result<()> {
    let Err(contents) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    for (path, contents) in &*contents {
        fs::write(path, contents)?;
    }

    *state = Ok(());
    Ok(())
}

pub fn read_dir(state: &mut Result<HashSet<PathBuf>, PathBuf>) -> io::Result<()> {
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

    *state = Ok(paths);
    Ok(())
}

pub fn read_file(state: &mut Result<Option<Vec<u8>>, PathBuf>) -> io::Result<()> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    let contents = fs::read(path)?;

    *state = Ok(Some(contents));
    Ok(())
}

pub fn read_files(
    state: &mut Result<Option<HashMap<PathBuf, Vec<u8>>>, HashSet<PathBuf>>,
) -> io::Result<()> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    let mut contents = HashMap::new();

    for path in &*paths {
        let content = fs::read(path)?;
        contents.insert(path.clone(), content);
    }

    *state = Ok(Some(contents));
    Ok(())
}

pub fn remove_dir(state: &mut Result<(), PathBuf>) -> io::Result<()> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::remove_dir_all(path)?;

    *state = Ok(());
    Ok(())
}

pub fn remove_dirs(state: &mut Result<(), HashSet<PathBuf>>) -> io::Result<()> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in &*paths {
        fs::remove_dir_all(path)?;
    }

    *state = Ok(());
    Ok(())
}

pub fn remove_file(state: &mut Result<(), PathBuf>) -> io::Result<()> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    fs::remove_file(path)?;

    *state = Ok(());
    Ok(())
}

pub fn remove_files(state: &mut Result<(), HashSet<PathBuf>>) -> io::Result<()> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    for path in &*paths {
        fs::remove_file(path)?;
    }

    *state = Ok(());
    Ok(())
}

pub fn rename(state: &mut Result<(), (PathBuf, PathBuf)>) -> io::Result<()> {
    let Err((from, to)) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    fs::rename(from, to)?;

    *state = Ok(());
    Ok(())
}
