use crate::{commands::ExecutableCommand, config};

pub struct ConfigCommand {
    pub config_name: String,
    pub config_value: String,
}

const POSSIBLE_CONFIG_NAMES: [&str; 4] = ["list", "base_branch", "user.name", "user.email"];

impl ExecutableCommand for ConfigCommand {
    fn run(&self) {
        if !POSSIBLE_CONFIG_NAMES.iter().any(|&v| v == self.config_name) {
            println!("This config does not exists!");
            std::process::exit(1);
        }

        let config = match config::Config::read_from_file() {
            Ok(config) => config,
            Err(e) => {
                println!("Error reading the config file: {}", e);
                std::process::exit(1);
            }
        };

        if self.config_name == *"list" {
            println!("{}", config);
            return;
        }

        self.print_or_update(config);
    }
}

impl ConfigCommand {
    fn print_or_update(&self, config: config::Config) {
        if self.config_value == *"" {
            match &self.config_name[..] {
                "base_branch" => println!("{}", config.base_branch),
                "user.name" => println!("{}", config.user.name),
                "user.email" => println!("{}", config.user.email),
                _ => (),
            }

            return;
        }

        let mut new_config = config.clone();

        match &self.config_name[..] {
            "base_branch" => new_config.base_branch = self.config_value.clone(),
            "user.name" => new_config.user.name = self.config_value.clone(),
            "user.email" => new_config.user.email = self.config_value.clone(),
            _ => (),
        }

        match config::Config::write_to_file(new_config) {
            Ok(_) => (),
            Err(e) => {
                println!("Something went wrong saving config: {}", e);
                std::process::exit(1);
            }
        }

        println!(
            "Config {} was updated successfully with {}",
            self.config_name, self.config_value
        );
    }
}
