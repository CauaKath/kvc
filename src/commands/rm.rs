use crate::{
    commands::ExecutableCommand,
    staging_area::StagingArea,
    utils::{get_file_path_relative_to_root, validate_path},
};
use std::{fs, path, process};

pub struct RmCommand {
    path: String,
    root_path: path::PathBuf,
}

impl ExecutableCommand for RmCommand {
    fn new(args: Vec<String>, root_folder: path::PathBuf) -> Self {
        let path = match args.first() {
            Some(v) => v,
            None => {
                println!("You must pass a path to remove from the index");
                std::process::exit(1);
            }
        };

        RmCommand {
            path: path.to_owned(),
            root_path: root_folder,
        }
    }

    fn run(&self) {
        let is_valid_path = validate_path(self.path.clone());
        if !is_valid_path {
            println!("The provided path is not valid to remove files from the index!");
            process::exit(1);
        }

        let path = &self.path;
        let file_metadata = fs::metadata(path).unwrap();

        if file_metadata.is_file() {
            self.process_file(path);
            process::exit(1);
        }

        self.process_dir(path);
    }
}

impl RmCommand {
    fn process_file(&self, path: &str) {
        let relative_path = get_file_path_relative_to_root(self.root_path.clone(), path.to_owned());

        let mut staging_area = StagingArea::open(self.root_path.clone());

        staging_area.remove(relative_path);
    }

    fn process_dir(&self, path: &str) {
        let read_dir = match fs::read_dir(path) {
            Ok(v) => v,
            Err(_e) => panic!("Could not read directory!"),
        };

        for entry in read_dir {
            let read_entry = entry.unwrap();
            let entry_path = read_entry.path();
            let entry_metadata = fs::metadata(&entry_path).unwrap();

            let path = entry_path.to_str().unwrap();
            if entry_metadata.is_file() {
                self.process_file(path);
                continue;
            }

            self.process_dir(path);
        }
    }
}
