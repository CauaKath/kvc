use std::{
    fs::{self},
    io::Write,
    path, process,
};

use crate::{
    commands::ExecutableCommand,
    constants::OBJECTS_FOLDER_NAME,
    staging_area::StagingArea,
    utils::{
        generate_hash, get_current_dir, get_file_path_relative_to_root, read_file, segment_hash,
    },
};

pub struct AddCommand {
    pub root_path: path::PathBuf,
    pub path: String,
}

impl ExecutableCommand for AddCommand {
    fn new(args: Vec<String>, root_folder: path::PathBuf) -> Self {
        let path = match args.first() {
            Some(v) => v,
            None => {
                println!("You must pass a path to add to the index");
                std::process::exit(1);
            }
        };

        AddCommand {
            path: path.to_owned(),
            root_path: root_folder,
        }
    }

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
        let file_content = read_file(path);
        let file_hash = match generate_hash(&file_content) {
            Ok(v) => v,
            Err(_e) => panic!("Could not generate file hash!"),
        };

        let (prefix, suffix) = segment_hash(&file_hash);

        self.create_prefix_dir(prefix);

        let mut file = self.create_suffix_file(prefix, suffix);

        match file.write_all(file_content.as_bytes()) {
            Ok(ok) => ok,
            Err(e) => panic!("Error writing to config file: {}", e),
        };

        self.add_file_to_index(path.to_owned(), file_hash);
    }

    fn add_file_to_index(&self, path: String, file_hash: String) {
        let mut staging_area = StagingArea::open(self.root_path.clone());

        let file_path_from_root = get_file_path_relative_to_root(self.root_path.clone(), path);

        staging_area.add(file_path_from_root, file_hash);
    }

    fn create_prefix_dir(&self, prefix: &str) {
        let path = self
            .root_path
            .clone()
            .join(OBJECTS_FOLDER_NAME)
            .join(prefix);

        if fs::exists(&path).expect("something went wrong!") {
            return;
        }

        match fs::create_dir(&path) {
            Ok(dir) => dir,
            Err(e) => panic!("Error on create prefix dir: {}", e),
        };
    }

    fn create_suffix_file(&self, prefix: &str, suffix: &str) -> fs::File {
        let path = self
            .root_path
            .clone()
            .join(OBJECTS_FOLDER_NAME)
            .join(prefix)
            .join(suffix);

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

        let cur_dir = get_current_dir();
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
