use clap::{Args};
use std::path::PathBuf;

// TODO: import exported Database into your own
#[derive(Debug, Args)]
pub struct ImportCommand {
    path: PathBuf,
    class: Option<String>,
}

pub fn import_commands() {
    println!("import");
}