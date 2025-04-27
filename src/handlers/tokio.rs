//! Module dedicated to the Tokio-based, async I/O handler.

use std::{
    collections::{HashMap, HashSet},
    io,
    path::PathBuf,
};

use tokio::fs;

use crate::Io;

/// The standard, blocking I/O handler.
///
/// This handler makes use of standard module [`std::io`] and Tokio
/// module [`tokio::io`] to manage files and directories.
pub async fn handle(io: Io<'_>) -> io::Result<()> {
    match io {
        Io::CreateDir(state) => create_dir(state).await,
        Io::CreateDirs(state) => create_dirs(state).await,
        Io::CreateFile(state) => create_file(state).await,
        Io::CreateFiles(state) => create_files(state).await,
        Io::ReadDir(state) => read_dir(state).await,
        Io::ReadFile(state) => read_file(state).await,
        Io::ReadFiles(state) => read_files(state).await,
        Io::RemoveDir(state) => remove_dir(state).await,
        Io::RemoveDirs(state) => remove_dirs(state).await,
        Io::RemoveFile(state) => remove_file(state).await,
        Io::RemoveFiles(state) => remove_files(state).await,
        Io::Rename(state) => rename(state).await,
    }
}

pub async fn create_dir(state: &mut Result<(), PathBuf>) -> io::Result<()> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::create_dir(path).await?;

    *state = Ok(());
    Ok(())
}

pub async fn create_dirs(state: &mut Result<(), HashSet<PathBuf>>) -> io::Result<()> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in &*paths {
        fs::create_dir(path).await?;
    }

    *state = Ok(());
    Ok(())
}

pub async fn create_file(state: &mut Result<(), (PathBuf, Vec<u8>)>) -> io::Result<()> {
    let Err((path, contents)) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    fs::write(path, contents).await?;

    *state = Ok(());
    Ok(())
}

pub async fn create_files(state: &mut Result<(), HashMap<PathBuf, Vec<u8>>>) -> io::Result<()> {
    let Err(contents) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    for (path, contents) in &*contents {
        fs::write(path, contents).await?;
    }

    *state = Ok(());
    Ok(())
}

pub async fn read_dir(state: &mut Result<HashSet<PathBuf>, PathBuf>) -> io::Result<()> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    let mut paths = HashSet::new();
    let mut dir = fs::read_dir(path).await?;

    while let Some(entry) = dir.next_entry().await? {
        paths.insert(entry.path());
    }

    *state = Ok(paths);
    Ok(())
}

pub async fn read_file(state: &mut Result<Option<Vec<u8>>, PathBuf>) -> io::Result<()> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    let contents = fs::read(path).await?;

    *state = Ok(Some(contents));
    Ok(())
}

pub async fn read_files(
    state: &mut Result<Option<HashMap<PathBuf, Vec<u8>>>, HashSet<PathBuf>>,
) -> io::Result<()> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    let mut contents = HashMap::new();

    for path in &*paths {
        let content = fs::read(path).await?;
        contents.insert(path.clone(), content);
    }

    *state = Ok(Some(contents));
    Ok(())
}

pub async fn remove_dir(state: &mut Result<(), PathBuf>) -> io::Result<()> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::remove_dir_all(path).await?;

    *state = Ok(());
    Ok(())
}

pub async fn remove_dirs(state: &mut Result<(), HashSet<PathBuf>>) -> io::Result<()> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in &*paths {
        fs::remove_dir_all(path).await?;
    }

    *state = Ok(());
    Ok(())
}

pub async fn remove_file(state: &mut Result<(), PathBuf>) -> io::Result<()> {
    let Err(path) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    fs::remove_file(path).await?;

    *state = Ok(());
    Ok(())
}

pub async fn remove_files(state: &mut Result<(), HashSet<PathBuf>>) -> io::Result<()> {
    let Err(paths) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    for path in &*paths {
        fs::remove_file(path).await?;
    }

    *state = Ok(());
    Ok(())
}

pub async fn rename(state: &mut Result<(), (PathBuf, PathBuf)>) -> io::Result<()> {
    let Err((from, to)) = state else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    fs::rename(from, to).await?;

    *state = Ok(());
    Ok(())
}
