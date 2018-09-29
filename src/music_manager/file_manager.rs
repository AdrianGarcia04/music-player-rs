extern crate dirs;

use std::{io, path, fs, fs::DirEntry, slice::Iter};

pub struct FileManager {
    directory: path::PathBuf,
    music_files: Vec<path::PathBuf>,
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
            directory: path,
            music_files: Vec::new(),
        }
    }

    pub fn songs(&self) -> Iter<path::PathBuf>{
        self.music_files.iter()
    }

    pub fn directory(&self) -> &path::PathBuf {
        &self.directory
    }

    pub fn from_dir(directory: &str) -> FileManager {
        let mut path = path::PathBuf::new();
        path.push(directory);
        FileManager {
            directory: path,
            music_files: Vec::new(),
        }
    }

    pub fn search_songs(&mut self) -> Result<(), io::Error>{
        match fs::read_dir(&self.directory) {
            Ok(files) => {
                for file in files {
                    self.save_file(file?);
                }
                Ok(())
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    fn save_file(&mut self, file: DirEntry) {
        let file_path_buf = file.path();
        let file_path_buf_clone = file_path_buf.clone();
        let file_path = file_path_buf.as_path();
        match file_path.extension() {
            Some(extension) => {
                if extension.eq("mp3") {
                    self.music_files.push(file_path_buf_clone);
                }
            },
            None => {}
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
