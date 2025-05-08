use crate::services::database;
use clap::builder::Str;
use clap::Args;
use procfs::process::Process;
use std::process::Command;
// TODO: implement command execution as well as subcommand execution, add how often a command should be played
#[derive(Debug, Args)]
pub struct ExecuteCommand {
    name: String,
    amount: Option<u32>,
}

pub async fn execute(execute_command: ExecuteCommand) {
    println!("this is your command {}", execute_command.name);
    let pool = database::connect_database().await.unwrap();
    let commands = database::find_all_subcommands(&pool, &execute_command.name)
        .await
        .unwrap();
    for command in commands {
        Command::new(command.command).output().expect("");
    }
}

fn get_current_shell() {
    let me = Process::myself().unwrap();
    println!("Current process: {:?}", me.stat());

    // Get parent PID (Unix-like systems)
    #[cfg(unix)]
    {
        let parent_pid = unsafe { libc::getppid() };
        println!("Parent PID: {}", parent_pid);

        // Try to get parent process info
        if let Ok(parent_proc) = Process::new(parent_pid) {
            println!("Parent process: {:?}", parent_proc.stat().unwrap().comm);
        } else {
            println!("Failed to get parent process info (may have exited)");
        }
    }

    #[cfg(not(unix))]
    {
        println!("Parent PID detection is only supported on Unix-like systems");
    }
}
