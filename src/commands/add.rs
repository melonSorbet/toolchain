use clap::Args;
use clap::ArgGroup;
use sqlx::SqlitePool;
use crate::{models, services};
use crate::services::database;
use Vec;

// TODO: function that opens up input and lets you input commands until certain key combination is
#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("subcommands")
        .args(&["multiple_commands", "add_subcommand"])
))]
pub struct AddCommand {
    name: String,
    #[arg(long = "multiple-commands", short = 'm')]
    multiple_commands: bool,
    #[arg(long = "subcommand", short = 's')]
    add_subcommand: bool,
    class: Option<String>,
}

pub async fn add(command: AddCommand){
    let database_path = database::database_path();
    let pool = services::database::connect_database(database_path).await.unwrap();
    if command.multiple_commands
    {
        add_multiple(&pool,  &command).await;
        return;
    }
    if command.add_subcommand {
        println!("add subcommand");
        add_subcommand(&pool,&command).await;
        return;
    }
    add_command(&pool, &command).await;
    println!("could successfully add command to database");

}
pub async fn add_subcommand(pool: &SqlitePool, command: &AddCommand){
    let input = open_input();
    let subcommands: Vec<models::command::Command> = services::database::find_all_subcommands(pool,&command.name).await.unwrap();

    let mut highest_index = 0;
    for subcommand in subcommands{
        if highest_index < subcommand.sorting_order {
            highest_index = subcommand.sorting_order;
        }
    }
    println!("highest sorting order: {}", highest_index);


    services::database::add_subcommand(&pool,command.name.clone(),models::command::Command{
        id: None,
        command: input,
        sorting_order: highest_index + 1,
        pipeline_id: command.name.clone(),
    }).await.expect("Couldnt add subcommand");
}
pub async fn add_command(pool: &SqlitePool, command: &AddCommand){
    let input = open_input();
    services::database::add_command(&pool, models::pipeline::Pipeline {
        id: command.name.clone(),
        description: "".to_string(),
        class: "".to_string(),
    }).await.expect("Couldnt add command");

    services::database::add_subcommand(&pool,command.name.clone(),models::command::Command{
        id: None,
        command: input,
        sorting_order: 1,
        pipeline_id: command.name.clone(),
    }).await.expect("Couldnt add subcommand");
}

pub async fn add_multiple(pool: &SqlitePool,command: &AddCommand) {
    let vec_input = open_continuous_input();
    services::database::add_command(&pool, models::pipeline::Pipeline{
        id: command.name.clone(),
        description: "".to_string(),
        class: "".to_string(),
    }).await.expect("Couldnt add command");
    let mut index = 1;

    for string in vec_input{
        services::database::add_subcommand(&pool,command.name.clone(),models::command::Command{
            id: None,
            command: string.clone(),
            sorting_order: index,
            pipeline_id: command.name.clone(),
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
