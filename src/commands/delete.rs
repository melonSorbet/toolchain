use clap::Args;
use clap::builder::Str;
use crate::services::database;

// TODO: find specific command-name in database and delete its entry
#[derive(Debug, Args)]
pub struct DeleteCommand {
    name: String,
    #[arg(long = "subcommand", short = 's')]
    subcommand_index: Option<u32>,
}
pub async fn delete(command: DeleteCommand) {
    match command.subcommand_index {
        Some(subcommand_index) => delete_subcommand(command, subcommand_index).await,
        None => delete_command(command).await,
    }
}

pub async fn delete_command(command: DeleteCommand) {
    let pool = database::connect_database().await.unwrap();
    database::delete_all_subcommands(&pool, &command.name).await.unwrap();
    database::delete_command(&pool, &command.name).await.unwrap();
    println!("Command {} deleted.", command.name);

}
pub async fn delete_subcommand(command: DeleteCommand, subcommand_index: u32) {
    let pool = database::connect_database().await.unwrap();
    database::delete_specific_subcommand(&pool, &command.name, &subcommand_index).await.expect("Could not delete entry in database");
    println!("Command {} from {} deleted.", subcommand_index,command.name);
}
