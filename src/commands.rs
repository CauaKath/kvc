use add::AddCommand;
use clap::Parser;
use config::ConfigCommand;
use core::fmt;
use help::HelpCommand;
use init::InitCommand;
use std::path;

use crate::utils::{get_current_dir, get_kvc_root_folder};

mod add;
mod config;
mod help;
mod init;

pub trait ExecutableCommand {
    fn run(&self);
    fn new(args: Vec<String>, root_folder: path::PathBuf) -> Self
    where
        Self: Sized;
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum Command {
    Init,
    Help,
    Config,
    Add,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Help => write!(f, "Help"),
            Command::Init => write!(f, "Init"),
            Command::Config => write!(f, "Config"),
            Command::Add => write!(f, "Add"),
        }
    }
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(value_enum, default_value_t=Command::Help)]
    pub command: Command,
    pub args: Vec<String>,
}

impl Cli {
    fn get_executable(
        command: Command,
        args: Vec<String>,
        root_folder: path::PathBuf,
    ) -> Box<dyn ExecutableCommand> {
        match command {
            Command::Init => Box::new(InitCommand::new(args, root_folder)),
            Command::Config => Box::new(ConfigCommand::new(args, root_folder)),
            Command::Help => Box::new(HelpCommand::new(args, root_folder)),
            Command::Add => Box::new(AddCommand::new(args, root_folder)),
        }
    }

    pub fn execute(command: Command, args: Vec<String>) {
        let cur_dir_path = get_current_dir();
        let (is_kvc_repo, root_folder) = get_kvc_root_folder(cur_dir_path);

        let commands_that_outside_kvc_repo: [Command; 2] = [Command::Init, Command::Help];
        if !commands_that_outside_kvc_repo.contains(&command) {
            if !is_kvc_repo {
                let not_kvc_repo_msg = "This is not a KVC repository!".to_owned()
                    + "\n\nUse `kvc init` to start a repository here.";

                println!("{}", not_kvc_repo_msg);
                std::process::exit(1);
            }
        }

        let executable = Self::get_executable(command, args, root_folder);

        executable.run();
    }
}
