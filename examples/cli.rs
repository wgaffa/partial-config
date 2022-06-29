use std::collections::HashSet;
use std::path::PathBuf;

use partial_config::{config, Build};
use partial_functional::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub enum ColorMode {
    Always,
    Never,
    #[default]
    Auto,
}

config!{
    #[derive(Debug, Clone)]
    pub struct Config {
        debug: { Any, bool },
        verbose: { Sum<u32>, u32 },
        output_file: { Last<PathBuf>, Option<PathBuf> },
        color: { Last<ColorMode>, ColorMode },
        files: { HashSet<PathBuf>, HashSet<PathBuf> },
    }
}

fn read_args() -> Config<Build> {
    todo!()
}

fn read_env() -> Config<Build> {
    todo!()
}

fn read_file() -> Config<Build> {
    todo!()
}

fn main() {
}
