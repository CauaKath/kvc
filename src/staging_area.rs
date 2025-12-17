use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path,
};

pub struct StagingArea {
    values: HashMap<String, String>,
    root_path: path::PathBuf,
}

const INDEX_FILE_PATH: &str = ".kvc/index";

impl StagingArea {
    fn open_file(&self, write: bool) -> fs::File {
        let mut index_path = self.root_path.clone();
        index_path.extend(&[INDEX_FILE_PATH]);

        let file = match fs::OpenOptions::new()
            .write(write)
            .read(true)
            .open(&index_path)
        {
            Ok(v) => v,
            Err(e) => panic!("Could not open index file! {}", e),
        };

        file
    }

    pub fn open(root_path: path::PathBuf) -> StagingArea {
        let mut staging_area = Self {
            values: HashMap::new(),
            root_path,
        };

        staging_area.read();

        staging_area
    }

    pub fn read(&mut self) -> HashMap<String, String> {
        let mut file = self.open_file(false);

        let mut file_content = String::new();
        match file.read_to_string(&mut file_content) {
            Ok(v) => v,
            Err(_e) => panic!("Could not read index content!"),
        };

        if file_content == "" {
            return self.values.clone();
        }

        for line in file_content.split("\n") {
            let (key, value) = Self::get_key_value(line);

            self.values.insert(key.to_owned(), value.to_owned());
        }

        self.values.clone()
    }

    fn get_key_value(str: &str) -> (&str, &str) {
        let mut splitted = str.split("||");

        let key = splitted.next().expect("No key was found!");
        let value = splitted.next().expect("No value was found!");

        (key, value)
    }

    fn save(&self) {
        let mut file = self.open_file(true);
        let mut file_content: String = "".to_owned();

        let mut idx = 0;
        for (key, value) in self.values.iter() {
            if idx == 0 {
                file_content = format!("{}||{}", &key, &value);
            } else {
                file_content = format!("{}\n{}||{}", file_content, &key, &value);
            }

            idx += 1;
        }

        match file.write_all(file_content.as_bytes()) {
            Ok(v) => v,
            Err(_e) => panic!("Could not write to index file!"),
        }
    }

    pub fn add(&mut self, path: String, hash: String) {
        self.values.insert(path, hash);

        self.save();
    }

    pub fn _remove(&mut self, path: String) {
        self.values.remove(&path);

        self.save();
    }
}
