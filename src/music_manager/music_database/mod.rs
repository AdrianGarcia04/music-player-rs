pub mod file_manager;
pub mod music_file;

use self::{music_file::MusicFile, file_manager::FileManager};
use postgres::{Connection, TlsMode, error, types::ToSql, rows::Rows};
use std::{io::{Error, ErrorKind}, slice::Iter};
use id3::Timestamp;

type PostgresError = error::Error;

pub struct MusicDatabase {
    connection: Option<Connection>,
    username: Option<String>,
    password: Option<String>,
    host: Option<String>,
    database: Option<String>,
    file_manager: FileManager,
}

impl MusicDatabase {

    pub fn new() -> MusicDatabase {
        MusicDatabase {
            connection: None,
            username: None,
            password: None,
            host: None,
            database: None,
            file_manager: FileManager::new()
        }
    }

    pub fn with_username(&mut self, username: &str) -> &mut MusicDatabase {
        self.username = Some(username.to_owned());
        self
    }

    pub fn with_password(&mut self, password: &str) -> &mut MusicDatabase {
        self.password = Some(password.to_owned());
        self
    }

    pub fn with_host(&mut self, host: &str) -> &mut MusicDatabase {
        self.host = Some(host.to_owned());
        self
    }

    pub fn with_database(&mut self, database: &str) -> &mut MusicDatabase {
        self.database = Some(database.to_owned());
        self
    }

    pub fn mine(&mut self) -> Result<(), Error>{
        self.file_manager.search_songs()?;
        self.save_albums()?;
        self.save_performers()?;
        for song in self.file_manager.songs() {
            self.save_song(song)?;
        }
        Ok(())
    }

    pub fn songs(&self) -> Iter<MusicFile> {
        self.file_manager.songs()
    }

    pub fn connect(&mut self) -> Result<(), PostgresError> {
        let username = match self.username {
            Some(ref username) => username,
            None => "postgres",
        };
        let password = match self.password {
            Some(ref password) => password,
            None => "postgres",
        };
        let host = match self.host {
            Some(ref host) => host,
            None => "0.0.0.0",
        };
        let database = match self.database {
            Some(ref database) => database,
            None => "music_player_rs",
        };
        let connection_url = format!("postgresql://{}:{}@{}/{}", username, password, host, database);
        info!(target: "MusicDatabase", "Connecting to {}", connection_url);
        self.connection = Some(Connection::connect(connection_url, TlsMode::None)?);
        info!(target: "MusicDatabase", "Succesfully connected to database");
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        match self.connection {
            Some(ref connection) => {
                connection.is_active()
            },
            None => {
                false
            }
        }
    }

    pub fn execute(&self, query: &str, params: &[&ToSql]) -> Result<u64, PostgresError> {
        match self.connection {
            Some(ref connection) => connection.execute(query, params),
            None => {
                let error = Error::new(ErrorKind::NotConnected, "Not connected to database");
                Err(PostgresError::from(error))
            }
        }
    }

    pub fn query(&self, query: &str, params: &[&ToSql]) -> Result<Rows, PostgresError> {
        match self.connection {
            Some(ref connection) => connection.query(query, params),
            None => {
                let error = Error::new(ErrorKind::NotConnected, "Not connected to database");
                Err(PostgresError::from(error))
            }
        }
    }

    fn save_albums(&mut self) -> Result<(), PostgresError>{
        for album in self.file_manager.albums() {
            let album_path = album.to_str().unwrap().to_string();
            let album_name = String::from(album.file_name().unwrap().to_str().unwrap());
            let query = format!("INSERT INTO albums (path, name, year) VALUES ('{}', '{}', 2018);",
                album_path, album_name);
            info!(target: "MusicDatabase", "Saving album: {:?}", album.file_name());
            self.query(&query, &[])?;
        }
        Ok(())
    }

    fn save_performers(&mut self) -> Result<(), PostgresError>{
        for song in self.file_manager.songs() {
            let performer = match song.artist() {
                Some(performer) => performer,
                None => "Unknown",
            };
            let query = format!("INSERT INTO performers (id_type, name) VALUES (2, '{}');",
                performer);
            info!(target: "MusicDatabase", "Saving performer: {:?}", performer);
            self.query(&query, &[])?;

        }
        Ok(())
    }

    pub fn save_song(&self, song: &MusicFile) -> Result<(), PostgresError> {
        let values = self.song_as_values(&song);
        let title = match song.title() {
            Some(title) => title,
            None => "",
        };
        let query = format!("INSERT INTO rolas (id_performer, id_album, path, title, track, year, \
        genre) VALUES {};", values);
        info!(target: "MusicDatabase", "Inserting {}", title);
        self.query(&query, &[])?;
        Ok(())
    }

    fn song_as_values(&self, song: &MusicFile) -> String {
        let performer = match song.artist() {
            Some(performer) => performer,
            None => "",
        };
        let id_performer = self.foreign_key("performer", "name", &performer);

        let album = match song.album() {
            Some(album) => album,
            None => "",
        };
        let id_album = self.foreign_key("album", "name", &album);

        let path = song.path();
        let title = match song.title() {
            Some(title) => title,
            None => "",
        };
        let track = match song.track() {
            Some(track) => track,
            None => &0,
        };
        let date_recorded = match song.date_recorded() {
            Some(date_recorded) => date_recorded,
            None => &Timestamp{ year: 2018, month: None, day: None, hour: None, minute: None,
                second: None },
        };
        let genre = match song.genre() {
            Some(genre) => genre,
            None => "Unknown",
        };
        format!("({}, {}, '{}', '{}', {}, {}, '{}')", id_performer, id_album, path, title, track,
        date_recorded.to_string(), genre)
    }

    fn foreign_key(&self, table: &str, column: &str, column_value: &str) -> String {
        let query = format!("SELECT id_{} FROM {}s WHERE {}='{}';", table, table, column, column_value);
        let rows = self.query(&query, &[]).unwrap();
        if rows.is_empty() {
            self.insert_and_get_id(table, column, column_value)
        }
        else {
            let table = format!("id_{}", table);
            let id: i32 = rows.get(0).get(&table[..]);
            id.to_string()
        }
    }

    fn insert_and_get_id(&self, table: &str, column: &str, column_value: &str) -> String {
        let query = format!("INSERT INTO {}s ({}) VALUES ('{}') RETURNING id_{}", table, column, column_value, table);
        let rows = self.query(&query, &[]).unwrap();
        let table = format!("id_{}", table);
        let id: i32 = rows.get(0).get(&table[..]);
        id.to_string()
    }

}
