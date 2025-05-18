use crate::services::database;
use clap::builder::Str;
use clap::{Args, Error};
use procfs::process::Process;
use std::process::Command;
// TODO: implement command execution as well as subcommand execution, add how often a command should be played
#[derive(Debug, Args)]
pub struct ExecuteCommand {
    name: String,
    amount: Option<u32>,
}

pub async fn execute(execute_command: ExecuteCommand) {
    let database_path = database::database_path();
    let pool = database::connect_database(database_path).await.unwrap();
    let commands = database::find_all_subcommands(&pool, &execute_command.name)
        .await
        .unwrap();
    let shell = get_current_shell().unwrap();
    println!("{}", shell);

    //TODO: Add Commandline Parsing as well as input
    for command in commands {
        let result = Command::new("bash")
            .arg("-c")
            .arg(command.command)
            .output()
            .expect("couldnt execute your wanted command");

        let stdout = String::from_utf8_lossy(&result.stdout);

        println!("{}", stdout);
    }
}

fn get_current_shell() -> Result<String, Box<dyn std::error::Error>> {
    let me = Process::myself()?;
    println!("Current process: {:?}", me.stat()?);

    #[cfg(unix)]
    {
        let parent_pid = unsafe { libc::getppid() };
        println!("Parent PID: {}", parent_pid);

        if let Ok(parent_proc) = Process::new(parent_pid) {
            let parent_stat = parent_proc.stat()?;
            println!("Parent process: {:?}", parent_stat.comm);
            return Ok(parent_stat.comm);
        } else {
            println!("Failed to get parent process info (may have exited)");
            return Err("Failed to get parent process".into());
        }
    }

    #[cfg(not(unix))]
    {
        println!("Parent PID detection is only supported on Unix-like systems");
        return Err("Unsupported platform".into());
    }
}
