use std::{
    env, fs,
    io::{BufReader, Error, Read},
    os::unix::fs::MetadataExt,
    path::{self, Path},
    str::from_utf8,
};

use sha2::{Digest, Sha256};

const ROOT_FOLDER_NAME: &str = ".kvc";

pub fn read_file(path: &str) -> (String, String) {
    let file_size = check_file_size(path);

    let relative_path = Path::new(path);
    let formatted_path = format!("{}", relative_path.display());

    if file_size > 400 {
        return (read_large_file(path), formatted_path);
    }

    (read_tiny_file(path), formatted_path)
}

fn check_file_size(path: &str) -> u64 {
    let file_metadata = fs::metadata(path).unwrap();

    file_metadata.size()
}

pub fn check_is_kvc_repo() -> bool {
    let cur_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => panic!("something went wrong: {}", e),
    };

    let mut root_dir_path = path::PathBuf::from(&cur_dir);
    root_dir_path.extend(&[ROOT_FOLDER_NAME]);

    if fs::exists(&root_dir_path).expect("something went wrong!") {
        return true;
    }

    false
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
