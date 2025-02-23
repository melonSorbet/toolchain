use clap::Args;
use clap::builder::Str;
use crate::services::database;
use std::process::Command;

// TODO: implement command execution as well as subcommand execution, add how often a command should be played
#[derive(Debug, Args)]
pub struct ExecuteCommand {
    name: String,
    amount: Option<u32>
}

pub async fn execute(execute_command: ExecuteCommand) {
    println!("this is your command {}", execute_command.name);
    let pool = database::connect_database().await.unwrap();
    let commands = database::find_all_subcommands(&pool,&execute_command.name).await.unwrap();
    for command in commands{
        Command::new(command.command).output().expect("");

    }
}
