extern crate music_player_rs;

use music_player_rs::music_manager::{music_database::MusicDatabase};

fn main() {
    let mut music_database = MusicDatabase::new();
    music_database.connect().unwrap();
    match music_database.save_songs() {
        Ok(_) => {},
        Err(e) => println!("{:?}", e)
    }
}
