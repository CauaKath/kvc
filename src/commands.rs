use add::AddCommand;
use clap::Parser;
use config::ConfigCommand;
use core::fmt;
use help::HelpCommand;
use init::InitCommand;

use crate::utils::{get_current_dir, get_kvc_root_folder};

mod add;
mod config;
mod help;
mod init;

pub trait ExecutableCommand {
    fn run(&self);
}

#[derive(clap::ValueEnum, Clone, Debug)]
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
    pub fn execute(command: Command, args: Vec<String>) {
        let cur_dir_path = get_current_dir();
        let (is_kvc_repo, root_folder) = get_kvc_root_folder(cur_dir_path);

        match command {
            Command::Init | Command::Help => (),
            _ => {
                if !is_kvc_repo {
                    let not_kvc_repo_msg = "This is not a KVC repository!".to_owned()
                        + "\n\nUse `kvc init` to start a repository here.";

                    println!("{}", not_kvc_repo_msg);
                    std::process::exit(1);
                }
            }
        }

        match command {
            Command::Help => {
                let command_name = match args.first() {
                    Some(v) => v,
                    None => {
                        let none_command_msg =
                            "You can use the help command to explain what other commands can do.".to_owned() +
                            " Ex: `kvc help init` " +
                            "will show you how this command works and if it support any arguments." +
                            "\n\n" +
                            "The current available commands are:" +
                            "\n\n" +
                            "- init" +
                            "\n" +
                            "- config";

                        println!("{}", none_command_msg);
                        std::process::exit(1);
                    }
                };

                let help_command = HelpCommand {
                    command_name: command_name.to_owned(),
                };

                help_command.run();
            }
            Command::Init => {
                let init_command = InitCommand {};

                init_command.run();
            }
            Command::Config => {
                let config_name = match args.first() {
                    Some(value) => value,
                    None => {
                        println!("You must pass which config you're trying to access/change");
                        std::process::exit(1);
                    }
                };

                let config_value = match args.get(1) {
                    Some(value) => value,
                    None => &String::from(""),
                };

                let config_command = ConfigCommand {
                    config_name: config_name.to_owned(),
                    config_value: config_value.to_owned(),
                };

                config_command.run();
            }
            Command::Add => {
                let path = match args.first() {
                    Some(v) => v,
                    None => {
                        println!("You must pass a path to add to the index");
                        std::process::exit(1);
                    }
                };

                let add_command = AddCommand {
                    path: path.to_owned(),
                    root_path: root_folder,
                };

                add_command.run();
            }
        }
    }
}
