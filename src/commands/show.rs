use clap::Args;

// TODO: show specific commands definition and subcommands to see what is does
#[derive(Debug, Args)]
pub struct ShowCommand {
    name: String,
}

pub fn show_commands(command: ShowCommand) {
    println!("show");
}
