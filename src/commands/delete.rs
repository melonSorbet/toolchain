use clap::Args;

// TODO: find specific command-name in database and delete its entry
#[derive(Debug, Args)]
pub struct DeleteCommand {
    name: String,
    multiple_commands: Option<String>,
}

pub fn delete_command(command: DeleteCommand) {
    println!("delete");
}
