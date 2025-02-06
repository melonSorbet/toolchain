use clap::{Parser, Subcommand};
use sqlx::Error;
use commands::{add, delete, export, import, modify, show};

mod commands {
    pub mod add;
    pub mod delete;
    pub mod export;
    pub mod import;
    pub mod modify;
    pub mod show;
}
mod models {
    pub mod command;
    pub mod subcommand;
}
mod services{
    pub mod database;
    pub mod hash;
}
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
#[tokio::main]
async fn main() -> Result<(), Error> {
    services::database::create_database().await;
    services::database::migrate_database().await.expect("couldnt lol");

    let args = Command::parse();
    println!("{:?}", &args);
    match args.command_type {
        CommandType::Add(add) => add::add_command(add).await,
        CommandType::Delete(delete) => delete::delete_command(delete),
        CommandType::Export(export) => export::export_commands(export),
        CommandType::Import(import) => import::import_commands(import),
        CommandType::Modify(modify) => modify::modify_commands(modify),
        CommandType::Show(show) => show::show_commands(show).await,
    }
    Ok(())
}
