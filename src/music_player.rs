extern crate music_player_rs;

use music_player_rs::music_manager::{file_manager};

fn main() {
    let mut file_manager = file_manager::FileManager::new();
    file_manager.search_songs().unwrap();

    for song in file_manager.songs() {
        println!("{:?}", song);
    }
}
