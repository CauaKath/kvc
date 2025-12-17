use std::{
    env, fs,
    io::{BufReader, Error, Read},
    os::unix::fs::MetadataExt,
    path,
    str::from_utf8,
};

use sha2::{Digest, Sha256};

use crate::constants::ROOT_FOLDER_NAME;

pub fn read_file(path: &str) -> String {
    let file_size = check_file_size(path);

    if file_size > 400 {
        return read_large_file(path);
    }

    read_tiny_file(path)
}

fn check_file_size(path: &str) -> u64 {
    let file_metadata = fs::metadata(path).unwrap();

    file_metadata.size()
}

pub fn get_current_dir() -> path::PathBuf {
    let cur_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => panic!("something went wrong: {}", e),
    };

    let root_dir_path = path::PathBuf::from(&cur_dir);

    root_dir_path
}

pub fn get_kvc_root_folder(path: path::PathBuf) -> (bool, path::PathBuf) {
    let mut cloned_path = path.clone();

    cloned_path.extend(&[ROOT_FOLDER_NAME]);

    loop {
        if fs::exists(&cloned_path).expect("something went wrong!") {
            break;
        }

        cloned_path = path.clone();
        let popped = cloned_path.pop();
        if !popped {
            return (false, path::PathBuf::new());
        }

        let parent_path_exists = match fs::exists(&cloned_path) {
            Ok(_v) => true,
            Err(_e) => false,
        };

        if !parent_path_exists {
            return (false, path::PathBuf::new());
        }

        return get_kvc_root_folder(cloned_path);
    }

    (true, path)
}

pub fn get_file_path_relative_to_root(root_path: path::PathBuf, path: String) -> String {
    let cur_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => panic!("something went wrong: {}", e),
    };

    let cur_dir_path = cur_dir.to_path_buf();
    let root_comp_path = match cur_dir_path.strip_prefix(root_path) {
        Ok(v) => format!("{}", v.display()),
        Err(e) => panic!("Could not strip file prefix! {}", e),
    };

    if root_comp_path == "" {
        format!("{}", path)
    } else {
        format!("{}/{}", root_comp_path, path)
    }
}

pub fn generate_hash(file_content: &str) -> Result<String, Error> {
    let result = Sha256::digest(file_content);
    let result_str = format!("{:x}", result);

    Ok(result_str)
}

fn read_tiny_file(path: &str) -> String {
    let mut file_input = match fs::File::open(path) {
        Ok(v) => v,
        Err(_e) => panic!("Could not read file!"),
    };

    let mut read_file = String::new();
    file_input
        .read_to_string(&mut read_file)
        .unwrap_or_default();

    read_file
}

fn read_large_file(path: &str) -> String {
    let file_input = fs::File::open(path).unwrap();
    let mut reader = BufReader::new(file_input);
    let mut buffer = [0_u8; 1024];

    let mut read_file = String::new();
    loop {
        let count = reader.read(&mut buffer).unwrap();

        if count == 0 {
            break;
        }

        let read_slice = match from_utf8(&buffer[..count]) {
            Ok(v) => v.to_owned(),
            Err(_e) => "".to_owned(),
        };

        if read_slice == "" {
            break;
        }

        read_file = format!("{}{}", read_file, read_slice);
    }

    read_file
}

pub fn segment_hash(hash: &str) -> (&str, &str) {
    hash.split_at(2)
}
