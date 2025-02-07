use clap::Args;
use crate::services::database;

// TODO: find specific command-name in database and delete its entry
#[derive(Debug, Args)]
pub struct DeleteCommand {
    name: String,
}

pub async fn delete_command(command: DeleteCommand) {
    let pool = database::connect_database().await.unwrap();
    database::delete_all_subcommands(&pool, &command.name).await.unwrap();
    database::delete_command(&pool, &command.name).await.unwrap();
    println!("Command {} deleted.", command.name);

}
