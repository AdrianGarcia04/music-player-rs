extern crate dirs;

use std::{io, path};
use std::fs;

pub struct FileManager {
    directory: path::PathBuf,
}

impl FileManager {

    pub fn new() -> FileManager {
        let path;
        match get_default_music_folder_path() {
            Ok(path_buf) => {
                path = path_buf;
            },
            Err(e) => {
                println!("{:?}", e);
                panic!();
            },
        };

        FileManager {
            directory: path
        }
    }

    pub fn from_dir(directory: &str) -> FileManager {
        let mut path = path::PathBuf::new();
        path.push(directory);
        FileManager {
            directory: path
        }
    }

    pub fn list_songs(&self) {
        match fs::read_dir(&self.directory) {
            Ok(files) => {
                for file in files {
                    println!("{:?}", file);
                }
            },
            Err(_) => {
                println!("An error ocurred");
            }
        }

    }
}

fn get_default_music_folder_path() -> Result<path::PathBuf, io::Error> {
    if let Some(mut home_dir) = dirs::home_dir() {
        home_dir.push("Music");
        Ok(home_dir.to_path_buf())
    }
    else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))
    }
}
