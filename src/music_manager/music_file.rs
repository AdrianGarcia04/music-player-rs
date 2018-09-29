use std::{path, fmt};
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
}

impl fmt::Display for MusicFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.path)
    }
}
