use clap::Args;

// TODO: function that opens up input and lets you input commands until certain key combination is
#[derive(Debug, Args)]
pub struct AddCommand {
    name: String,
    multiple_commands: Option<String>,
    class: Option<String>,
}

pub fn add_command(command: AddCommand) {
    let input = open_input();
    println!("this is your command, name {}, command {}", command.name,  input);
}

fn open_input() -> String{
    let mut string_allocator = String::new();
    std::io::stdin().read_line(&mut string_allocator).expect("lalal");
    return string_allocator;
}
// TODO: add class system to categorize and make it easier to save commands.
