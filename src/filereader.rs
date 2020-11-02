use std::fs;
use std::io::prelude::*;

pub struct FileReader {
    path: String
}

// Static
impl FileReader {
    pub fn new(path: String) -> FileReader {
        return FileReader {
            path
        }
    }
}

// Instance
impl FileReader {
    

    #[allow(dead_code)]
    pub fn to_string_with_default(&self, default: String) -> String {
        let result = fs::File::open(self.path.clone());
        match result {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_v) => return contents,
                    _ => return default
                }
            },
            _ => return default
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> Option<String> {
        let result = fs::File::open(self.path.clone());
        match result {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_v) => return Some(contents),
                    _ => return None
                }
            },
            _ => return None
        }
    }
}