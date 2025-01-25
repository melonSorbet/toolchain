use clap::{Args};

// TODO: open up interface to modify description name and sub-commands of defined command
#[derive(Debug,Args)]
pub struct ModifyCommand {
    name: String,
}

pub fn modify_commands() {}