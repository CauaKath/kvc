use crate::commands::ExecutableCommand;
use crate::config::{Config, User};
use core::panic;
use std::env;
use std::fs;
use std::io::Write;
use std::path;

const ROOT_FOLDER_NAME: &str = ".kvc";

pub struct InitCommand;

impl ExecutableCommand for InitCommand {
    fn run(&self) {
        let cur_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => panic!("something went wrong: {}", e),
        };

        Self::create_root_folder(&cur_dir);
        Self::create_refs_folder(&cur_dir);
        Self::create_objects_folder(&cur_dir);
        Self::create_head_file(&cur_dir);
        Self::create_config_file(&cur_dir);
        Self::create_index_file(&cur_dir);
        Self::hide_root_folder(&cur_dir);

        println!("A kvc repository was created!");
    }
}

impl InitCommand {
    fn create_root_folder(cur_dir: &path::PathBuf) {
        let mut root_dir_path = path::PathBuf::from(&cur_dir);
        root_dir_path.extend(&[ROOT_FOLDER_NAME]);

        if fs::exists(&root_dir_path).expect("something went wrong!") {
            println!("This directory is a kvc repository already!");
            std::process::exit(1);
        }

        match fs::create_dir(&root_dir_path) {
            Ok(dir) => dir,
            Err(e) => panic!("Error on create root dir: {}", e),
        };
    }

    fn hide_root_folder(cur_dir: &path::PathBuf) {
        let mut root_dir_path = path::PathBuf::from(&cur_dir);
        root_dir_path.extend(&[ROOT_FOLDER_NAME]);

        if hf::is_hidden(&root_dir_path).unwrap_or_default() {
            std::process::exit(1);
        }

        hf::hide(&root_dir_path).unwrap();
    }

    fn create_refs_folder(cur_dir: &path::PathBuf) {
        const REFS_FOLDER_NAME: &str = "refs";

        let mut folder_path = path::PathBuf::from(&cur_dir);
        folder_path.extend(&[ROOT_FOLDER_NAME, REFS_FOLDER_NAME]);

        match fs::create_dir(&folder_path) {
            Ok(dir) => dir,
            Err(e) => panic!("Error on create refs folder: {}", e),
        }
    }

    fn create_objects_folder(cur_dir: &path::PathBuf) {
        const OBJECTS_FOLDER_NAME: &str = "objects";

        let mut folder_path = path::PathBuf::from(&cur_dir);
        folder_path.extend(&[ROOT_FOLDER_NAME, OBJECTS_FOLDER_NAME]);

        match fs::create_dir(&folder_path) {
            Ok(dir) => dir,
            Err(e) => panic!("Error on create objects folder: {}", e),
        }
    }

    fn create_head_file(cur_dir: &path::PathBuf) {
        const HEAD_FILE_NAME: &str = "HEAD";

        let mut file_path = path::PathBuf::from(&cur_dir);
        file_path.extend(&[ROOT_FOLDER_NAME, HEAD_FILE_NAME]);

        let mut file = match fs::File::create(file_path) {
            Ok(file) => file,
            Err(e) => panic!("Error creating the HEAD file: {}", e),
        };

        let file_content: &str = "ref: refs/master";

        match file.write_all(file_content.as_bytes()) {
            Ok(ok) => ok,
            Err(e) => panic!("Error writing to HEAD file: {}", e),
        }
    }

    fn create_config_file(cur_dir: &path::PathBuf) {
        const CONFIG_FILE_NAME: &str = "config";

        let mut file_path = path::PathBuf::from(&cur_dir);
        file_path.extend(&[ROOT_FOLDER_NAME, CONFIG_FILE_NAME]);

        let mut file = match fs::File::create(&file_path) {
            Ok(file) => file,
            Err(e) => panic!("Error creating the config file: {}", e),
        };

        const DEFAULT_BASE_BRANCH: &str = "master";

        let default_config = Config {
            base_branch: DEFAULT_BASE_BRANCH.to_string(),
            user: User {
                name: "".to_string(),
                email: "".to_string(),
            },
        };

        let file_content: &str = &default_config.to_string();

        match file.write_all(file_content.as_bytes()) {
            Ok(ok) => ok,
            Err(e) => panic!("Error writing to config file: {}", e),
        };
    }

    fn create_index_file(cur_dir: &path::PathBuf) {
        const INDEX_FILE_NAME: &str = "index";

        let mut file_path = path::PathBuf::from(&cur_dir);
        file_path.extend(&[ROOT_FOLDER_NAME, INDEX_FILE_NAME]);

        match fs::File::create(file_path) {
            Ok(file) => file,
            Err(e) => panic!("Error creating the index file: {}", e),
        };
    }
}
