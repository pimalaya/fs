#![cfg(feature = "std")]

use std::{
    io::{stdin, stdout, Write as _},
    time::Instant,
};

use fs_flows::{flows::CreateFiles, handlers::std::handle};
use tempdir::TempDir;

fn main() {
    env_logger::init();

    let tmp = TempDir::new("std-create-files").unwrap();

    let n = read_line("How many temp files to create?")
        .parse::<usize>()
        .unwrap();

    let start = Instant::now();

    let mut flow = CreateFiles::new(
        (0..n).map(|n| (tmp.path().join(n.to_string()), b"Hello, world!".to_vec())),
    );

    while let Err(io) = flow.next() {
        handle(io).unwrap();
    }

    let duration = start.elapsed();

    println!("Created {n} temp files in {duration:?}!");
}

fn read_line(prompt: &str) -> String {
    print!("{prompt} ");
    stdout().flush().unwrap();
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_owned()
}
