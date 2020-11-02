use std::fs::File;
use std::io::prelude::*;

pub struct FileWriter {
    path: String
}

impl FileWriter {
    pub fn new(path: String) -> FileWriter {
        return FileWriter {
            path
        }
    }
}

impl FileWriter {
    #[allow(dead_code)]
    pub fn write_or_update(&self, content: String) {
        match File::create(self.path.clone()) {
            Ok(mut file) => {
                match file.write_all(content.as_bytes()) {
                    _ => return ()
                }
            },
            _ => return ()
        }
        //file.write_all(b"Hello, world!")?;
    }
}