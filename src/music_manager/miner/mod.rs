extern crate dirs;

/// Music file module.
pub mod music_file;

use super::music_database::MusicDatabase;
use self::music_file::MusicFile;
use std::{io, path, fs, sync::mpsc};

/// A miner instance has a directory (where the mine is done), a database connection,
/// a list of listeners, and the number of files in the directory (scanned and not scanned).
/// The miner is able use it's database connection to store the music it founds.
pub struct Miner {
    directory: path::PathBuf,
    database: MusicDatabase,
    listeners: Vec<mpsc::Sender<MinerEvent>>,
    number_of_files: f64,
    files_scanned: f64,
}

impl Miner {

    /// Creates a new instance of a miner, making it's connection to the database available.
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
            listeners: Vec::new(),
            number_of_files: 0.0,
            files_scanned: 0.0
        }
    }

    /// Returns the current directory where the mining is done.
    pub fn directory(&self) -> &path::PathBuf {
        &self.directory
    }

    /// Creates a new instance of a miner, with an specific mining directory.
    pub fn from_dir(directory: &str) -> Miner {
        let mut path = path::PathBuf::new();
        path.push(directory);
        let mut database = MusicDatabase::new();
        database.connect().unwrap();
        Miner {
            directory: path,
            database: database,
            listeners: Vec::new(),
            number_of_files: 0.0,
            files_scanned: 0.0

        }
    }

    /// Searches music files inside the directory, and notifies listeners when the mining is
    /// running, a music file is stored and when the miner finishes.
    pub fn mine(&mut self) -> Result<(), io::Error> {
        let directory = self.directory.clone();
        self.number_of_files = self.count_files(&directory);
        self.notify_listeners(MinerEvent::Mining);
        self.mine_from_dir(&directory)?;
        self.notify_listeners(MinerEvent::Finished);
        Ok(())
    }

    /// Mines recursively from an specific directory.
    pub fn mine_from_dir(&mut self, directory: &path::Path) -> Result<(), io::Error> {
        info!(target: "Miner", "Searching songs in {:?}", directory);
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.database.save_album(path.clone()).unwrap();
                self.mine_from_dir(&path)?;
            }
            else {
                self.save_song(entry);
                self.files_scanned = self.files_scanned + 1.0;
                let percentage = self.files_scanned / self.number_of_files;
                self.notify_listeners(MinerEvent::Percentage(percentage));
                info!(target: "Miner", "Percentage mined {:?}", percentage);
            }
        }
        Ok(())
    }

    /// Given a file, if it is music file, stores its information in database.
    pub fn save_song(&mut self, file: fs::DirEntry) {
        let path = file.path();
        let path_clone = path.clone();

        match path_clone.as_path().extension() {
            Some(extension) => {
                if extension.eq("mp3") {
                    info!(target: "Miner", "Found song {:?}", path.clone());
                    self.database.save_song(MusicFile::from_path(path)).unwrap();
                }
                else {
                    info!(target: "Miner", "Ignoring {:?}", path.clone());
                }
            },
            None => {}
        }
    }

    /// Returns a new miner event listener.
    pub fn get_listener(&mut self) -> mpsc::Receiver<MinerEvent> {
        let (tx, rx) = mpsc::channel();
        self.listeners.push(tx);
        rx
    }

    /// Notifies the miner's listeners about an event.
    pub fn notify_listeners(&mut self, event: MinerEvent) {
        for listener in self.listeners.iter_mut() {
            listener.send(event.clone()).unwrap();
        }
    }

    /// Counts recursively the number of music files in a directory.
    pub fn count_files(&self, directory: &path::Path) -> f64 {
        info!(target: "Miner", "Counting songs in {:?}", directory);
        let mut songs = 0.0;
        for entry in fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                songs = songs + self.count_files(&path);
            }
            else {
                match path.as_path().extension() {
                    Some(extension) => {
                        if extension.eq("mp3") {
                            songs = songs + 1.0;
                        }
                    },
                    None => {}
                }
            }
        }
        info!(target: "Miner", "Songs found in {:?}: {:?}", directory, songs);
        songs
    }
}

/// Returns the default music folder of the computer.
pub fn get_default_music_folder_path() -> Result<path::PathBuf, io::Error> {
    if let Some(mut home_dir) = dirs::home_dir() {
        home_dir.push("Music");
        Ok(home_dir.to_path_buf())
    }
    else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))
    }
}

#[derive(Clone, Debug)]
/// Types of events that occur during the mining.
pub enum MinerEvent {
    Ready,
    Mining,
    Percentage(f64),
    Finished,
}
