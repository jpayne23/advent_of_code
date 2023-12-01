use std::fs;
use std::path::Path;

pub mod file {

    pub fn input_file_to_iter(input_path: &str) -> Vec<String> {
        let contents = fs::read_to_string(Path::new(input_path)).expect("Couldn't read file");
        return contents.split("\n").map(|el| el.to_string()).collect();
    }
}
