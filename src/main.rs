mod commands;
use clap::{Parser, Subcommand};
use commands::{add, delete, export, import, modify, show};

#[derive(Debug, Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub command_type: CommandType,
}

#[derive(Debug, Subcommand)]
enum CommandType {
    Add(add::AddCommand),
    Delete(delete::DeleteCommand),
    Export(export::ExportCommand),
    Import(import::ImportCommand),
    Modify(modify::ModifyCommand),
    Show(show::ShowCommand),
}

// TODO: implement smart and enhancible way to add more commands.
fn main() {
    println!("{:?}", Command::parse())
}
