extern crate music_player_rs;

use music_player_rs::music_manager::{file_manager, music_database};

fn main() {
    let mut file_manager = file_manager::FileManager::new();
    file_manager.search_songs().unwrap();

    let mut music_database = music_database::MusicDatabase::new();
    music_database.connect();

    for song in file_manager.songs() {
        // println!("{}", song);
        println!("{:?}", music_database.save_song(song));
    }


}
