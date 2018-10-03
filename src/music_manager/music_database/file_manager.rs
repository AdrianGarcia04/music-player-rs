extern crate dirs;

use super::{music_file::MusicFile};
use std::{io, path, fs, slice::Iter};

pub struct FileManager {
    directory: path::PathBuf,
    music_files: Vec<MusicFile>,
    albums: Vec<path::PathBuf>,
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
            albums: Vec::new(),
        }
    }

    pub fn songs(&self) -> Iter<MusicFile> {
        self.music_files.iter()
    }

    pub fn albums(&self) -> Iter<path::PathBuf> {
        self.albums.iter()
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
            albums: Vec::new(),
        }
    }

    pub fn search_songs(&mut self) -> Result<(), io::Error> {
        let directory = self.directory.clone();
        self.search_songs_from_dir(&directory)?;
        Ok(())
    }

    pub fn search_songs_from_dir(&mut self, directory: &path::Path) -> Result<(), io::Error> {
        info!(target: "FileManager", "Searching songs in {:?}", directory);
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.albums.push(path.clone());
                self.search_songs_from_dir(&path)?;
            }
            else {
                self.save_file(entry);
            }
        }
        Ok(())
    }

    fn save_file(&mut self, file: fs::DirEntry) {
        let path = file.path();
        let path_clone = path.clone();

        match path_clone.as_path().extension() {
            Some(extension) => {
                if extension.eq("mp3") {
                    info!(target: "FileManager", "Found song {:?}", path.clone());
                    self.music_files.push(MusicFile::from_path(path));
                }
                else {
                    info!(target: "FileManager", "Ignoring {:?}", path.clone());
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
