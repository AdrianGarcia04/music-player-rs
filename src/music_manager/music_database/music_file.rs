use std::{path};
use id3::{Tag, Timestamp};

pub struct MusicFile {
    path: path::PathBuf,
    artist: Option<String>,
    title: Option<String>,
    album: Option<String>,
    date_recorded: Option<Timestamp>,
    genre: Option<String>,
    track: Option<u32>,
}

impl MusicFile {

    pub fn from_path(path: path::PathBuf) -> MusicFile {
        let tag = Tag::read_from_path(path.clone()).unwrap();
        MusicFile {
            path: path,
            artist: tag.artist().map(str::to_string),
            title: tag.title().map(str::to_string),
            album: tag.album().map(str::to_string),
            date_recorded: tag.date_recorded(),
            genre: tag.genre().map(str::to_string),
            track: tag.track(),
        }
    }

    pub fn path(&self) -> String {
        format!("'{:?}'", self.path)
    }

    pub fn artist(&self) -> &Option<String> {
        &self.artist
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    pub fn album(&self) -> &Option<String> {
        &self.album
    }

    pub fn date_recorded(&self) -> &Option<Timestamp> {
        &self.date_recorded
    }

    pub fn genre(&self) -> &Option<String> {
        &self.genre
    }

    pub fn track(&self) -> &Option<u32> {
        &self.track
    }

}
