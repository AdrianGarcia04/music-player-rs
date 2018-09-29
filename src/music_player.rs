extern crate music_player_rs;

use music_player_rs::music_manager::{file_manager};

fn main() {
    let file_manager = file_manager::FileManager::new();
    file_manager.list_songs();

    let file_manager = file_manager::FileManager::from_dir("./test_songs/");
    file_manager.list_songs();
}
