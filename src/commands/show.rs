use crate::{services, services::database};
use clap::Args;
// TODO: show specific commands definition and subcommands to see what is does
#[derive(Debug, Args)]
pub struct ShowCommand {
    name: String,
}

pub async fn show_commands(show_command: ShowCommand) {
    let database_path = database::database_path();
    let pool = services::database::connect_database(database_path)
        .await
        .unwrap();
    let command = services::database::find_pipeline(&pool, &show_command.name)
        .await
        .unwrap();
    let subcommands = services::database::find_all_commands(&pool, &show_command.name)
        .await
        .unwrap();

    println!(
        "command:\n  name: {}\n  description: {}\n  class: {}",
        command.id, command.description, command.class
    );
    println!("subcommands:");
    for subcommand in subcommands {
        println!(
            "  - index: {}\n    command: {}\n    id: {}\n    pipeline_id: {}",
            subcommand.sorting_order,
            subcommand.command,
            subcommand.id.unwrap_or_default(),
            subcommand.pipeline_id
        );
    }
}
