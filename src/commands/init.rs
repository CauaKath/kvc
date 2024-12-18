use core::panic;
use std::env;
use std::fs;
use std::io::Write;
use std::path;

const ROOT_FOLDER_NAME: &str = ".kvc";

pub struct InitCommand;

impl InitCommand {
    pub fn run() {
        let cur_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => panic!("something went wrong: {}", e),
        };

        create_root_folder(&cur_dir);

        let file_content = "teste";
        let mut file_path = path::PathBuf::from(&cur_dir);
        file_path.extend(&[ROOT_FOLDER_NAME, "teste.txt"]);

        let mut file = match fs::File::create(file_path) {
            Ok(file) => file,
            Err(e) => panic!("Error creating the file: {}", e),
        };

        match file.write_all(file_content.as_bytes()) {
            Ok(ok) => ok,
            Err(e) => panic!("Error writing to file: {}", e),
        }

        println!("A kvc repository was created!")
    }
}

pub fn create_root_folder(cur_dir: &path::PathBuf) {
    let mut root_dir_path = path::PathBuf::from(&cur_dir);
    root_dir_path.extend(&[ROOT_FOLDER_NAME]);

    if fs::exists(&root_dir_path).expect("something went wrong!") {
        println!("This directory is a kvc repository already!");
        std::process::exit(1);
    }

    match fs::create_dir(&root_dir_path) {
        Ok(dir) => dir,
        Err(e) => panic!("Error on created root dir: {}", e),
    };

    hf::hide(&root_dir_path).unwrap();
}
