use std::{
    env,
    fs::{self},
    io::Write,
    path, process,
};

use crate::{
    commands::ExecutableCommand,
    staging_area::StagingArea,
    utils::{generate_hash, read_file, segment_hash},
};

pub struct AddCommand {
    pub path: String,
}

const ROOT_FOLDER_NAME: &str = ".kvc";
const OBJECTS_FOLDER_NAME: &str = "objects";

impl ExecutableCommand for AddCommand {
    fn run(&self) {
        let valid_path = self.validate_path();

        if !valid_path {
            println!("The provided path is not valid to add files to the index!");
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

impl AddCommand {
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

    fn process_file(&self, path: &str) {
        let (file_content, file_path) = read_file(path);
        let file_hash = match generate_hash(&file_content) {
            Ok(v) => v,
            Err(_e) => panic!("Could not generate file hash!"),
        };

        let (prefix, suffix) = segment_hash(&file_hash);
        let cur_dir_path = self.get_cur_dir_path();

        let mut prefix_dir_path = cur_dir_path.clone();
        prefix_dir_path.extend(&[ROOT_FOLDER_NAME, OBJECTS_FOLDER_NAME, prefix]);

        self.create_prefix_dir(prefix_dir_path);

        let mut suffix_file_path = cur_dir_path.clone();
        suffix_file_path.extend(&[ROOT_FOLDER_NAME, OBJECTS_FOLDER_NAME, prefix, suffix]);

        let mut file = self.create_suffix_file(suffix_file_path);

        match file.write_all(file_content.as_bytes()) {
            Ok(ok) => ok,
            Err(e) => panic!("Error writing to config file: {}", e),
        };

        let mut staging_area = StagingArea::open();

        staging_area.add(file_path, file_hash);
    }

    fn get_cur_dir_path(&self) -> path::PathBuf {
        let cur_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => panic!("something went wrong: {}", e),
        };

        path::PathBuf::from(&cur_dir)
    }

    fn create_prefix_dir(&self, path: path::PathBuf) {
        if fs::exists(&path).expect("something went wrong!") {
            return;
        }

        match fs::create_dir(&path) {
            Ok(dir) => dir,
            Err(e) => panic!("Error on create root dir: {}", e),
        };
    }

    fn create_suffix_file(&self, path: path::PathBuf) -> fs::File {
        let file_exists = match fs::exists(&path) {
            Ok(v) => v,
            Err(_e) => panic!("Could not create suffix file!"),
        };

        if file_exists {
            let file = match fs::OpenOptions::new().write(true).open(path) {
                Ok(v) => v,
                Err(_e) => panic!("Could not open suffix file!"),
            };

            return file;
        }

        let file = match fs::File::create(&path) {
            Ok(v) => v,
            Err(e) => panic!("Error on create root dir: {}", e),
        };

        file
    }

    fn validate_path(&self) -> bool {
        let is_path_valid = fs::exists(&self.path).unwrap_or_default();

        let cur_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => panic!("something went wrong: {}", e),
        };

        let cur_dir_abs_path = match path::absolute(&cur_dir) {
            Ok(v) => v,
            Err(_) => {
                println!("Something went wrong converting to absolute cur_dir!");
                process::exit(1);
            }
        };

        let args_abs_path = match path::absolute(&self.path) {
            Ok(v) => v,
            Err(_) => {
                println!("Something went wrong converting to absolute args path!");
                process::exit(1);
            }
        };

        if !is_path_valid || !args_abs_path.starts_with(cur_dir_abs_path.to_str().unwrap()) {
            return false;
        }

        return true;
    }
}
