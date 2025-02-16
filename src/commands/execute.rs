use clap::Args;
use clap::builder::Str;
use crate::services::database;
// TODO: implement command execution as well as subcommand execution, add how often a command should be played
#[derive(Debug, Args)]
pub struct ExecuteCommand {
    name: String,
    amount: Option<u32>
}

pub async fn execute(execute_command: ExecuteCommand) {
    println!("this is your command {}", execute_command.name);
}
