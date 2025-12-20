use crate::{
    staging_area::StagingArea,
    traits::ExecutableCommand,
    utils::{generate_hash, read_file},
};
use std::{
    collections::HashMap,
    fs,
    path::{self, PathBuf},
    process,
};

pub struct StatusCommand {
    root_path: path::PathBuf,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum FileStatus {
    Untracked,
    Staged,
    Modified,
}

impl ExecutableCommand for StatusCommand {
    fn new(_args: Vec<String>, root_folder: path::PathBuf) -> Self {
        Self {
            root_path: root_folder,
        }
    }

    fn run(&self) {
        let mut file_entries: HashMap<FileStatus, Vec<String>> = HashMap::from([
            (FileStatus::Untracked, Vec::new()),
            (FileStatus::Staged, Vec::new()),
            (FileStatus::Modified, Vec::new()),
        ]);

        let root_path = self.root_path.to_str().unwrap();

        self.process_dir(root_path, &mut file_entries);

        let untracked_files = file_entries.get(&FileStatus::Untracked).unwrap();
        let staged_files = file_entries.get(&FileStatus::Staged).unwrap();
        let modified_files = file_entries.get(&FileStatus::Modified).unwrap();

        if untracked_files.len() == 0 && staged_files.len() == 0 && modified_files.len() == 0 {
            println!("No changes made!");
            process::exit(1);
        }

        if staged_files.len() > 0 {
            println!("Files to be commited:");
            println!("  (use \"kvc rm <file>\" to remove file for commit)");
            for file in staged_files {
                println!("\t\x1b[32m{}\x1b[0m", file);
            }
            println!();
        }

        if modified_files.len() > 0 {
            println!("Files changed but not staged for commit:");
            println!("  (use \"kvc add <file>\" to update file for commit)");
            for file in modified_files {
                println!("\t\x1b[31m{}\x1b[0m", file);
            }
            println!();
        }

        if untracked_files.len() > 0 {
            println!("Untracked files:");
            println!("  (use \"kvc add <file>\" to add file for commit)");
            for file in untracked_files {
                println!("\t\x1b[31m{}\x1b[0m", file);
            }
            println!();
        }
    }
}

impl StatusCommand {
    fn process_file(&self, path: &str, files: &mut HashMap<FileStatus, Vec<String>>) {
        let file_content = read_file(path);
        let file_hash = match generate_hash(&file_content) {
            Ok(v) => v,
            Err(_e) => panic!("Could not generate file hash!"),
        };

        let relative_path = PathBuf::from(path);
        let str_relative_path = format!(
            "{}",
            relative_path
                .strip_prefix(self.root_path.clone())
                .unwrap()
                .display()
        );

        let staging_area = StagingArea::open(self.root_path.clone());
        let staged_hash = staging_area.get(str_relative_path.clone());
        if staged_hash == *"" {
            let untracked_files = files.get_mut(&FileStatus::Untracked).unwrap();
            untracked_files.push(str_relative_path.clone());
            return;
        }

        if staged_hash != file_hash {
            let modified_files = files.get_mut(&FileStatus::Modified).unwrap();
            modified_files.push(str_relative_path.clone());
        }

        let staged_files = files.get_mut(&FileStatus::Staged).unwrap();
        staged_files.push(str_relative_path.clone());
    }

    fn process_dir(&self, path: &str, files: &mut HashMap<FileStatus, Vec<String>>) {
        let read_dir = match fs::read_dir(path) {
            Ok(v) => v,
            Err(_e) => panic!("Could not read directory!"),
        };

        for entry in read_dir {
            let read_entry = entry.unwrap();
            let entry_path = read_entry.path();
            let entry_relative_path = entry_path.strip_prefix(self.root_path.clone()).unwrap();

            if entry_relative_path.starts_with(".kvc") {
                continue;
            }

            let entry_metadata = fs::metadata(&entry_path).unwrap();

            let path = format!("{}", entry_path.display());
            if entry_metadata.is_file() {
                self.process_file(&path, files);
                continue;
            }

            self.process_dir(&path, files);
        }
    }
}
