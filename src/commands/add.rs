use clap::Args;
use crate::models::command::Command;
use crate::{models, services};

// TODO: function that opens up input and lets you input commands until certain key combination is
#[derive(Debug, Args)]
pub struct AddCommand {
    name: String,
    multiple_commands: Option<String>,
    class: Option<String>,
}

pub async fn add_command(command: AddCommand){
    let input = open_input();
    let pool = services::database::connect_database().await.unwrap();

    services::database::add_command(&pool, Command {
        id: command.name.clone(),
        description: "".to_string(),
        class: "".to_string(),
    }).await.expect("Couldnt add command");

    services::database::add_subcommand(&pool,command.name.clone(),models::subcommand::Subcommand{
        subcommand_id: command.name.clone().to_string(),
        command: input,
        sorting_order: 1,
        command_id: command.name.clone(),
    }).await.expect("Couldnt add subcommand");

    println!("could successfully add command to database");

}
pub async fn add(command: AddCommand){

}
pub async fn add_multiple(command: AddCommand) {

}

fn open_input() -> String{
    let mut string_allocator = String::new();
    std::io::stdin().read_line(&mut string_allocator).expect("lalal");
    string_allocator
}
// TODO: add class system to categorize and make it easier to save commands.
