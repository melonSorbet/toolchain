use std::num::IntErrorKind::Empty;
use clap::Args;
use sqlx::SqlitePool;
use crate::models::command::Command;
use crate::{models, services};
use Vec;
// TODO: function that opens up input and lets you input commands until certain key combination is
#[derive(Debug, Args)]
pub struct AddCommand {
    name: String,
    #[arg(long = "multiple-commands", short = 'm')]
    multiple_commands: bool,
    class: Option<String>,
}

pub async fn add_command(command: AddCommand){

    let pool = services::database::connect_database().await.unwrap();
    if command.multiple_commands
    {
        add_multiple(&pool,  &command).await;
        return;
    }

    add(&pool, &command).await;
    println!("could successfully add command to database");

}
pub async fn add(pool: &SqlitePool, command: &AddCommand){
    let input = open_input();
    services::database::add_command(&pool, Command {
        id: command.name.clone(),
        description: "".to_string(),
        class: "".to_string(),
    }).await.expect("Couldnt add command");

    services::database::add_subcommand(&pool,command.name.clone(),models::subcommand::Subcommand{
        id: None,
        command: input,
        sorting_order: 1,
        command_id: command.name.clone(),
    }).await.expect("Couldnt add subcommand");
}
pub async fn add_multiple(pool: &SqlitePool,command: &AddCommand) {
    let vec_input = open_continuous_input();
    services::database::add_command(&pool, Command {
        id: command.name.clone(),
        description: "".to_string(),
        class: "".to_string(),
    }).await.expect("Couldnt add command");
    let mut index = 1;

    for string in vec_input{
        services::database::add_subcommand(&pool,command.name.clone(),models::subcommand::Subcommand{
            id: None,
            command: string.clone(),
            sorting_order: index,
            command_id: command.name.clone(),
        }).await.expect("Couldnt add subcommand");

        index += 1;
    }
    println!("add multiple command successfully");

}

fn open_input() -> String{
    let mut string_allocator = String::new();
    std::io::stdin().read_line(&mut string_allocator).expect("lalal");
    string_allocator
}
fn open_continuous_input() -> Vec<String>{
    let mut vector: Vec<String> = vec![];
    loop {
        let mut string_allocator = String::new();
        std::io::stdin().read_line(&mut string_allocator).expect("lalal");
        if string_allocator.trim() == "quit"{
            break;
        }
        vector.push(string_allocator);
    }
    vector
}

// TODO: add class system to categorize and make it easier to save commands.
