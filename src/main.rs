use clap::Parser;

#[derive(Parser)]
struct Command{
    command: String,
    name: String
}


// TODO: implement smart and enhancible way to add more commands.
fn main(){

    // TODO: read in all the arguments. With clap or own implementation.
    // TODO: take arguments and parse them into the correct command-function
    let args = Command::parse();


    println!("this is the command you want to execute {} ",args.command);
    println!("this is the name you want the command to have {} ",args.name);
}
