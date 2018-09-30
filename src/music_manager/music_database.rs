use super::{music_file::MusicFile};
use postgres::{Connection, TlsMode, error, types::ToSql, rows::Rows};
use std::io::{Error, ErrorKind};
type PostgresError = error::Error;

pub struct MusicDatabase {
    connection: Option<Connection>,
    username: Option<String>,
    password: Option<String>,
    host: Option<String>,
    database: Option<String>,
}

impl MusicDatabase {

    pub fn new() -> MusicDatabase {
        MusicDatabase {
            connection: None,
            username: None,
            password: None,
            host: None,
            database: None,
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
        self.connection = Some(Connection::connect(connection_url, TlsMode::None)?);
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

    pub fn save_song(&self, song: &MusicFile) -> Result<(), PostgresError > {
        let values = self.song_as_values(&song);
        let query = format!("INSERT INTO song (artist_id, album_id, genre_id, disc_id, title, \
            path, lyrics, year, duration, date_recorded, date_released) VALUES {};", values);
        self.query(&query, &[])?;
        Ok(())
    }

    fn song_as_values(&self, song: &MusicFile) -> String {
        let artist_id = self.foreign_key("artist", "name", &song.artist());
        let album_id = self.foreign_key("album", "name", &song.album());
        let genre_id = self.foreign_key("genre", "name", &song.genre());
        let disc_id = self.foreign_key("disc", "name", &song.disc());
        let title = song.title();
        let path = song.path();
        let lyrics = song.lyrics();
        let year = song.year();
        let duration = song.duration();
        let date_recorded = song.date_recorded();
        let date_released = song.date_released();
        format!("({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})", artist_id, album_id, genre_id,
            disc_id, title, path, lyrics, year, duration, date_recorded, date_released)
    }

    fn foreign_key(&self, table: &str, column: &str, column_value: &str) -> String {
        let query = format!("SELECT id FROM {} WHERE {}='{}'", table, column, column_value);
        let rows = self.query(&query, &[]).unwrap();
        if rows.is_empty() {
            self.insert_and_get_id(table, column, column_value)
        }
        else {
            let id: i32 = rows.get(0).get("id");
            id.to_string()
        }
    }

    fn insert_and_get_id(&self, table: &str, column: &str, column_value: &str) -> String {
        let query = format!("INSERT INTO {} ({}) VALUES ('{}') RETURNING id", table, column, column_value);
        let rows = self.query(&query, &[]).unwrap();
        let id: i32 = rows.get(0).get("id");
        id.to_string()
    }

}
