use crate::services::database;
use clap::Args;

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
    let database_path = database::database_path();
    let pool = database::connect_database(database_path).await.unwrap();
    database::delete_all_commands(&pool, &command.name)
        .await
        .unwrap();
    database::delete_command(&pool, &command.name)
        .await
        .unwrap();
    println!("Command {} deleted.", command.name);
}
pub async fn delete_subcommand(command: DeleteCommand, subcommand_index: u32) {
    let database_path = database::database_path();
    let pool = database::connect_database(database_path).await.unwrap();
    database::delete_specific_command(&pool, &command.name, &subcommand_index)
        .await
        .expect("Could not delete entry in database");
    println!(
        "Command {} from {} deleted.",
        subcommand_index, command.name
    );
}
