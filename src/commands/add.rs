use clap::Args;
use crate::{Command, CommandType};

// TODO: function that opens up input and lets you input commands until certain key combination is
#[derive(Debug, Args)]
pub struct AddCommand {
    name: String,
    multiple_commands: Option<String>,
}

pub fn add_command(command: Command) {
    println!("add")
}

// TODO: add class system to categorize and make it easier to save commands.
