use std::{
    fmt,
    fs::{self, OpenOptions},
    io::{self, Write},
    path,
};

pub struct User {
    pub email: String,
    pub name: String,
}

pub struct Config {
    pub user: User,
    pub base_branch: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "base_branch={}\nuser.email={}\nuser.name={}",
            self.base_branch, self.user.email, self.user.name,
        )
    }
}

const CONFIG_FILE_PATH: &str = ".kvc/config";

impl Config {
    pub fn clone(&self) -> Config {
        Config {
            base_branch: self.base_branch.clone(),
            user: User {
                name: self.user.name.clone(),
                email: self.user.email.clone(),
            },
        }
    }

    pub fn new() -> Config {
        Config {
            base_branch: String::from(""),
            user: User {
                name: String::from(""),
                email: String::from(""),
            },
        }
    }

    pub fn read_from_file() -> Result<Config, io::Error> {
        let config_file_path = path::PathBuf::from(CONFIG_FILE_PATH);

        let config_content = fs::read_to_string(&config_file_path).unwrap_or_default();

        if config_content == *"" {
            println!("Your config file is empty!");
            std::process::exit(1);
        }

        let mut config = Self::new();

        for line in config_content.split("\n") {
            let (key, value) = Self::get_key_value(line);

            match key {
                "base_branch" => config.base_branch = value.to_owned(),
                "user.name" => config.user.name = value.to_owned(),
                "user.email" => config.user.email = value.to_owned(),
                _ => continue,
            }
        }

        Ok(config)
    }

    fn get_key_value(str: &str) -> (&str, &str) {
        let mut splitted = str.split("=");

        let key = splitted.next().expect("No key was found!");
        let value = splitted.next().expect("No value was found!");

        (key, value)
    }

    pub fn write_to_file(config: Config) -> Result<(), io::Error> {
        let config_file_path = path::PathBuf::from(CONFIG_FILE_PATH);

        let mut file = match OpenOptions::new().write(true).open(&config_file_path) {
            Ok(file) => file,
            Err(_) => {
                println!("Error opening config file!");
                std::process::exit(1);
            }
        };

        let file_content = &config.to_string();

        match file.write_all(file_content.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                println!("Something went wrong writing to config file! {}", e);
                std::process::exit(1);
            }
        }

        Ok(())
    }
}
