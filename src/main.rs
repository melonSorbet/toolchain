mod commands;
use clap::{Parser, Subcommand};
use commands::{add, delete, export, import, modify, show};


#[derive(Debug, Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub command_type: CommandType,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    Add(add::AddCommand),
    Delete(delete::DeleteCommand),
    Export(export::ExportCommand),
    Import(import::ImportCommand),
    Modify(modify::ModifyCommand),
    Show(show::ShowCommand),
}

// TODO: implement smart and enhancible way to add more commands.
fn main() {
    let args = Command::parse();
    println!("{:?}", &args);
    match args.command_type {
        CommandType::Add(add) => add::add_command(add),
        CommandType::Delete(delete) => delete::delete_command(delete),
        CommandType::Export(export) => export::export_commands(export),
        CommandType::Import(import)=> import::import_commands(import),
        CommandType::Modify(modify)=> modify::modify_commands(modify),
        CommandType::Show(show)=> show::show_commands(show),
    }
}
