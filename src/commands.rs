use clap::Parser;
use core::fmt;
use init::InitCommand;

mod init;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Command {
    Init,
    Help,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Init => write!(f, "Init"),
            Command::Help => write!(f, "Help"),
        }
    }
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(value_enum, default_value_t=Command::Help)]
    pub command: Command,
}

impl Cli {
    pub fn execute(command: Command) {
        match command {
            Command::Init => InitCommand::run(),
            Command::Help => println!("Help"),
        }
    }
}
