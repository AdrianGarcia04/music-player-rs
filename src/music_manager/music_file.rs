use std::{path, fmt, string};
use id3::{Tag};

pub struct MusicFile {
    path: path::PathBuf,
    tag: Tag,
}

impl MusicFile {

    pub fn from_path(path: path::PathBuf) -> MusicFile {
        MusicFile {
            path: path.clone(),
            tag: Tag::read_from_path(path).unwrap(),
        }
    }

    pub fn artist(&self) -> String {
        match self.tag.artist() {
            Some(artist) => artist.to_owned(),
            None => String::from("")
        }
    }

    pub fn album(&self) -> String {
        match self.tag.album() {
            Some(album) => album.to_owned(),
            None => String::from("")
        }
    }

    pub fn genre(&self) -> String {
        match self.tag.genre() {
            Some(genre) => genre.to_owned(),
            None => String::from("")
        }
    }

    pub fn disc(&self) -> String {
        match self.tag.disc() {
            Some(disc) => disc.to_string(),
            None => String::from("")
        }
    }

    pub fn title(&self) -> String {
        match self.tag.title() {
            Some(title) => format!("'{}'", title),
            None => String::from("null")
        }
    }

    pub fn lyrics(&self) -> String {
        let mut lyrics = self.tag.lyrics();
        match lyrics.next() {
            Some(ref lyrics) => format!("'{}'", lyrics.text),
            None => String::from("null")
        }
    }

    pub fn year(&self) -> String {
        match self.tag.year() {
            Some(year) => year.to_string(),
            None => String::from("null")
        }
    }

    pub fn duration(&self) -> String {
        match self.tag.duration() {
            Some(duration) => duration.to_string(),
            None => String::from("null")
        }
    }

    pub fn date_recorded(&self) -> String {
        match self.tag.date_recorded() {
            Some(date_recorded) => date_recorded.to_string(),
            None => String::from("null")
        }
    }

    pub fn date_released(&self) -> String {
        match self.tag.date_released() {
            Some(date_released) => date_released.to_string(),
            None => String::from("null")
        }
    }
}

impl fmt::Display for MusicFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let title = self.title();
        let lyrics = self.lyrics();
        let year = self.year();
        let duration = self.duration();
        let date_recorded = self.date_recorded();
        let date_released = self.date_released();
        write!(f, "{}, {}, {}, {}, {}, {}", title, lyrics, year, duration, date_recorded, date_released)
    }
}
