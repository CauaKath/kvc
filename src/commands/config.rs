use std::{fmt, path};

use crate::{commands::ExecutableCommand, config, traits::FromString};

pub struct ConfigCommand {
    config_name: ConfigName,
    config_value: String,
}

#[derive(PartialEq)]
enum ConfigName {
    List,
    BaseBranch,
    UserName,
    UserEmail,
    Invalid,
}

impl FromString for ConfigName {
    fn from_string(s: String) -> Self {
        match &s[..] {
            "base_branch" => ConfigName::BaseBranch,
            "user.name" => ConfigName::UserName,
            "user.email" => ConfigName::UserEmail,
            "" => ConfigName::List,
            _ => ConfigName::Invalid,
        }
    }
}

impl fmt::Display for ConfigName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConfigName::List => write!(f, "list"),
            ConfigName::BaseBranch => write!(f, "base_branch"),
            ConfigName::UserName => write!(f, "user.name"),
            ConfigName::UserEmail => write!(f, "user.email"),
            ConfigName::Invalid => write!(f, "invalid"),
        }
    }
}

const POSSIBLE_CONFIG_NAMES: [ConfigName; 4] = [
    ConfigName::List,
    ConfigName::BaseBranch,
    ConfigName::UserName,
    ConfigName::UserEmail,
];

impl ExecutableCommand for ConfigCommand {
    fn new(args: Vec<String>, _root_folder: path::PathBuf) -> Self {
        let config_name = match args.first() {
            Some(value) => value,
            None => &String::new(),
        };

        let config_value = match args.get(1) {
            Some(value) => value,
            None => &String::from(""),
        };

        ConfigCommand {
            config_name: ConfigName::from_string(config_name.to_owned()),
            config_value: config_value.to_owned(),
        }
    }

    fn run(&self) {
        if self.config_name.to_string() != ""
            && !POSSIBLE_CONFIG_NAMES.iter().any(|v| *v == self.config_name)
        {
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

        if self.config_name == ConfigName::List {
            println!("{}", config);
            return;
        }

        self.print_or_update(config);
    }
}

impl ConfigCommand {
    fn print_or_update(&self, config: config::Config) {
        if self.config_value == *"" {
            match self.config_name {
                ConfigName::BaseBranch => println!("{}", config.base_branch),
                ConfigName::UserName => println!("{}", config.user.name),
                ConfigName::UserEmail => println!("{}", config.user.email),
                _ => (),
            }

            return;
        }

        let mut new_config = config.clone();

        match self.config_name {
            ConfigName::BaseBranch => new_config.base_branch = self.config_value.clone(),
            ConfigName::UserName => new_config.user.name = self.config_value.clone(),
            ConfigName::UserEmail => new_config.user.email = self.config_value.clone(),
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
