extern crate dirs;

use super::{music_file::MusicFile};
use std::{io, path, fs, fs::DirEntry, slice::Iter};

pub struct FileManager {
    directory: path::PathBuf,
    music_files: Vec<MusicFile>,
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

    pub fn songs(&self) -> Iter<MusicFile> {
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

    pub fn search_songs(&mut self) -> Result<(), io::Error> {
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

    pub fn search_songs_from_dir(&mut self, directory: &str) -> Result<(), io::Error> {
        match fs::read_dir(directory) {
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
        let path = file.path();
        let path_clone = path.clone();

        match path_clone.as_path().extension() {
            Some(extension) => {
                if extension.eq("mp3") {
                    self.music_files.push(MusicFile::from_path(path));
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
