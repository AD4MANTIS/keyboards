use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

pub struct LoggingOptions {
    pub text: bool,
    pub image: SaveImageOption,
    pub verbosity: Verbosity,
}

impl LoggingOptions {
    pub fn silent() -> Self {
        Self {
            text: false,
            image: SaveImageOption::None,
            verbosity: Verbosity::Silent,
        }
    }
}

pub enum Verbosity {
    Silent,
    Normal,
}

impl Verbosity {
    pub fn is_normal(&self) -> bool {
        match *self {
            Verbosity::Silent => false,
            Verbosity::Normal => true,
        }
    }
}

impl Default for Verbosity {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(PartialEq)]
pub enum SaveImageOption {
    None,
    Last,
    FirstAndLast,
    AllBest,
}

// SAVE SCORE
pub(super) fn append_to_file(path: &str, update_line: &str) {
    if !Path::new(path).exists() {
        File::create(path).unwrap_or_else(|_| panic!("Failed to create File {}", path));
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap_or_else(|_| panic!("Failed to open file {}", path));

    writeln!(file, "{}", update_line)
        .unwrap_or_else(|_| panic!("Failed to write to file {}", path));
}

pub(super) fn append_updates(update_line: &str) {
    append_to_file("results/iterationScores.txt", update_line);
}
