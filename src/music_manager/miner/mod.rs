extern crate dirs;
pub mod music_file;

use super::music_database::MusicDatabase;
use self::music_file::MusicFile;
use std::{io, path, fs};

pub struct Miner {
    directory: path::PathBuf,
    database: MusicDatabase,
}

impl Miner {

    pub fn new() -> Miner {
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

        let mut database = MusicDatabase::new();
        database.connect().unwrap();
        Miner {
            directory: path,
            database: database,
        }
    }

    pub fn directory(&self) -> &path::PathBuf {
        &self.directory
    }

    pub fn from_dir(directory: &str) -> Miner {
        let mut path = path::PathBuf::new();
        path.push(directory);
        let mut database = MusicDatabase::new();
        database.connect().unwrap();
        Miner {
            directory: path,
            database: database,
        }
    }

    pub fn mine(&mut self) -> Result<(), io::Error> {
        let directory = self.directory.clone();
        self.mine_from_dir(&directory)?;
        Ok(())
    }

    pub fn mine_from_dir(&mut self, directory: &path::Path) -> Result<(), io::Error> {
        info!(target: "Miner", "Searching songs in {:?}", directory);
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.database.save_album(path.clone());
                self.mine_from_dir(&path)?;
            }
            else {
                self.save_song(entry);
            }
        }
        Ok(())
    }

    fn save_song(&mut self, file: fs::DirEntry) {
        let path = file.path();
        let path_clone = path.clone();

        match path_clone.as_path().extension() {
            Some(extension) => {
                if extension.eq("mp3") {
                    info!(target: "Miner", "Found song {:?}", path.clone());
                    self.database.save_song(MusicFile::from_path(path));
                }
                else {
                    info!(target: "Miner", "Ignoring {:?}", path.clone());
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
