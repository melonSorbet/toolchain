use clap::Args;
use crate::{services,services::database};
// TODO: show specific commands definition and subcommands to see what is does
#[derive(Debug, Args)]
pub struct ShowCommand {
    name: String,
}

pub async fn show_commands(show_command: ShowCommand) {
    let database_path = database::database_path();
    let pool = services::database::connect_database(database_path).await.unwrap();
    let command = services::database::find_command(&pool, &show_command.name).await.unwrap();
    let subcommands = services::database::find_all_subcommands(&pool, &show_command.name).await.unwrap();

    println!("command name: [{}], command description: [{}], command class: [{}]", command.id,command.description,command.class);
    for subcommand in subcommands {
        println!("subcommand index: [{}], subcommand command: [{}], subcommand id: [{:?}], command id: [{}] ",subcommand.sorting_order, subcommand.command, subcommand.id.unwrap(), subcommand.pipeline_id );
    }
}
