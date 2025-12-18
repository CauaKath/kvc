use clap::Parser;
use commands::Cli;

mod commands;
mod config;
mod constants;
mod staging_area;
mod traits;
mod utils;

fn main() {
    let args = match Cli::try_parse() {
        Ok(res) => res,
        Err(..) => {
            println!("This command does not exist. Try kvc help.");
            std::process::exit(1);
        }
    };

    Cli::execute(args.command, args.args);
}
