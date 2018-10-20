use std::{path};
use id3::{Tag, Timestamp};

/// A music file has the music file path and the important information about a song, such as
/// artist, title, album, etc.
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

    /// Creates a new instance of a music file given the path of the file.
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

    /// Returns the music file's path.
    pub fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }

    /// Returns the music file's artist.
    pub fn artist(&self) -> &Option<String> {
        &self.artist
    }

    /// Returns the music file's title.
    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    /// Returns the music file's album.
    pub fn album(&self) -> &Option<String> {
        &self.album
    }

    /// Returns the music file's date_recorded.
    pub fn date_recorded(&self) -> &Option<Timestamp> {
        &self.date_recorded
    }

    /// Returns the music file's genre.
    pub fn genre(&self) -> &Option<String> {
        &self.genre
    }

    /// Returns the music file's track.
    pub fn track(&self) -> &Option<u32> {
        &self.track
    }

}
