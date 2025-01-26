use clap::{Args, Parser};
use std::path::PathBuf;

// TODO: export commands stored in database into json or other good exportable format
#[derive(Debug,Args)]
pub struct ExportCommand{
    path: Option<PathBuf>,
    class: Option<String>,
}

pub fn export_commands(command: ExportCommand) {
    println!("export");
}
