use std::path;

pub trait FromString {
    fn from_string(s: String) -> Self;
}

pub trait ExecutableCommand {
    fn run(&self);
    fn new(args: Vec<String>, root_folder: path::PathBuf) -> Self
    where
        Self: Sized;
}
