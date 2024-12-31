use std::{env, fs, path, process};

pub struct AddCommand {
    pub path: String,
}

impl AddCommand {
    pub fn run(&self) {
        let valid_path = self.validate_path();

        if !valid_path {
            println!("The provided path is not valid to add files to the index!");
            process::exit(1);
        }

        // TODO:
        // read all files on the passed path
        // foreach: create a hash based on file content
        // foreach: save the encrypted file content on: .kvc/objects/{prefix}/{suffix}
        // {prefix} = 2-first of hash
        // {suffix} = rest of hash
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

        true
    }
}
