use super::{query_manager, miner::music_file::MusicFile};
use super::query_manager::{
    TableColumn as TC,
    TableColumn::Rolas as Rolas,
    TableColumn::Performers as Performers,
    TableColumn::Albums as Albums,
    Conditional::Eq,
    Conditional::EqVal,
};
use std::{path, collections::HashMap};
use id3::Timestamp;
use sqlite;

type SQLiteError = sqlite::Error;

/// A music database stores the connection to the SQLite database and its name.
pub struct MusicDatabase {
    connection: Option<sqlite::Connection>,
    database: Option<String>,
}

impl MusicDatabase {

    /// Creates a new instance of a music database.
    pub fn new() -> MusicDatabase {
        MusicDatabase {
            connection: None,
            database: None,
        }
    }

    /// Given a database name, creates a new instance of a music database, with the corresponding
    /// SQLite database name.
    pub fn with_database(&mut self, database: &str) -> &mut MusicDatabase {
        self.database = Some(database.to_owned());
        self
    }

    /// Tries to connect to the SQLite database.
    pub fn connect(&mut self) -> Result<(), SQLiteError> {
        let database = match self.database {
            Some(ref database) => &database[..],
            None => "./music_player_rs.db",
        };
        let database_path = path::Path::new(database);
        let create_database = !database_path.exists();
        info!(target: "MusicDatabase", "Connecting to {:?}", database_path);
        self.connection = Some(sqlite::open(database_path)?);
        info!(target: "MusicDatabase", "Succesfully connected to database");
        if create_database {
            self.execute(&query_manager::create_database().unwrap())?;
        }
        Ok(())
    }

    /// Returns the active connection, if exists.
    pub fn connection(&self) -> Result<&sqlite::Connection, SQLiteError> {
        match self.connection {
            Some(ref connection) => Ok(connection),
            None => {
                Err(SQLiteError {
                    code: Some(1),
                    message: Some(String::from("Not connected to database"))
                })
            }
        }
    }

    /// Executes an SQL statement.
    pub fn execute(&self, query: &str) -> Result<(), SQLiteError> {
        let connection = self.connection()?;
        connection.execute(query)
    }

    /// Executes an SQL statement, and returns the resulting rows.
    pub fn query(&self, query: &str) -> Result<sqlite::Cursor, SQLiteError> {
        let connection = self.connection()?;
        Ok(connection.prepare(query)?.cursor())
    }

    /// Returns all the songs in database, following the order: title, performer, album and genre.
    pub fn songs(&self) -> Vec<HashMap<&str, String>> {
        let mut query = query_manager::select(
            &[Rolas("title"), Rolas("genre"), Performers("name"), Albums("name")],
            &[Eq(Rolas("id_performer"), Performers("id_performer")), Eq(Rolas("id_album"),
                Albums("id_album"))]
        );
        query += " ORDER BY rolas.title ASC";
        let mut cursor = self.query(&query).unwrap();
        let mut songs = Vec::new();
        while let Some(row) = cursor.next().unwrap() {
            let mut hashmap: HashMap<&str, String> = HashMap::new();
            let title = row[0].as_string().unwrap();
            let genre = row[1].as_string().unwrap();
            let performer = row[2].as_string().unwrap();
            let album = row[3].as_string().unwrap();
            hashmap.insert("title", title.to_owned());
            hashmap.insert("performer", performer.to_owned());
            hashmap.insert("album", album.to_owned());
            hashmap.insert("genre", genre.to_owned());
            songs.push(hashmap);
        }
        songs
    }

    /// Given an album path, creates a new album entry in the database.
    pub fn save_album(&mut self, album: path::PathBuf) -> Result<(), SQLiteError> {
        if self.album_in_database(&album) {
            return Ok(());
        }
        let album_path = album.to_str().unwrap().to_string();
        let album_name = String::from(album.file_name().unwrap().to_str().unwrap());
        let query = format!("INSERT INTO albums (path, name, year) VALUES ('{}', '{}', 2018);",
            album_path, album_name);
        info!(target: "MusicDatabase", "Inserting album {:?}", album.file_name().unwrap());
        self.execute(&query)?;
        Ok(())
    }

    /// Given a music file, creates a new performer entry in the database.
    fn save_performer(&self, song: &MusicFile) -> Result<(), SQLiteError>{
        let performer = match song.artist() {
            Some(performer) => performer,
            None => "Unknown",
        };
        let query = format!("INSERT INTO performers (id_type, name) VALUES (2, '{}');",
            performer);
        info!(target: "MusicDatabase", "Inserting performer {:?}", performer);
        self.execute(&query)?;
        Ok(())
    }

    /// Given a music file, creates a new "rolas" entry in the database, storing it performer
    /// and album.
    pub fn save_song(&self, song: MusicFile) -> Result<(), SQLiteError> {
        if self.song_in_database(&song) {
            return Ok(());
        }
        self.save_performer(&song)?;
        let values = self.song_as_values(&song);
        let title = match song.title() {
            Some(title) => title,
            None => "",
        };
        let query = format!("INSERT INTO rolas (id_performer, id_album, path, title, track, year, \
        genre) VALUES {};", values);
        info!(target: "MusicDatabase", "Inserting song {}", title);
        self.execute(&query)?;
        Ok(())
    }

    /// Given a music file, returns a string with all the values to be inserted in the database.
    pub fn song_as_values(&self, song: &MusicFile) -> String {
        let performer = match song.artist() {
            Some(performer) => performer,
            None => "Unknown",
        };
        let id_performer = self.foreign_key("performer", "name", &performer);

        let album = match song.album() {
            Some(album) => album,
            None => "Unknown",
        };
        let id_album = self.foreign_key("album", "name", &album);

        let path = song.path();
        let title = match song.title() {
            Some(title) => title,
            None => "Unknown",
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

    /// Given the table, column and value, returns the corresponding row id.
    /// If the value does not exists in database, it is inserted.
    pub fn foreign_key(&self, table: &str, column: &str, column_value: &str) -> i64 {
        let column_query = format!("id_{}", table);
        let select_table = TC::from_str(table, &column_query).unwrap();
        let where_table_column = TC::from_str(table, column).unwrap();
        let conditional = EqVal(where_table_column, column_value);

        let query = query_manager::select(&[select_table], &[conditional]);
        let mut cursor = self.query(&query).unwrap();
        if let Some(row) = cursor.next().unwrap() {
            row[0].as_integer().unwrap()
        }
        else {
            self.insert_and_get_id(table, column, column_value)
        }
    }

    /// Given the table, column and value, creates the corresponding new entry in the database.
    pub fn insert_and_get_id(&self, table: &str, column: &str, column_value: &str) -> i64 {
        let query = format!("INSERT INTO {}s ({}) VALUES ('{}');", table, column, column_value);
        self.execute(&query).unwrap();
        let query = format!("SELECT last_insert_rowid();");
        let mut cursor = self.query(&query).unwrap();
        if let Some(row) = cursor.next().unwrap() {
            row[0].as_integer().unwrap()
        }
        else {
            0
        }
    }

    /// Checks if the given song exists in database.
    pub fn song_in_database(&self, song: &MusicFile) -> bool {
        let title = match song.title() {
            Some(title) => title,
            None => "Unknown",
        };
        let query = query_manager::select(
            &[Rolas("id_rola")],
            &[EqVal(Rolas("title"), title)]
        );
        let mut cursor = self.query(&query).unwrap();
        cursor.next().unwrap().is_some()
    }

    /// Checks if the given album exists in database.
    pub fn album_in_database(&self, album: &path::PathBuf) -> bool {
        let album_path = album.to_str().unwrap().to_string();
        let query = query_manager::select(
            &[Albums("id_album")],
            &[EqVal(Albums("path"), &album_path)]
        );
        let mut cursor = self.query(&query).unwrap();
        cursor.next().unwrap().is_some()
    }

    /// Given an SQL statement, returns the resulting rows of the query as a strings vector.
    pub fn search_songs(&self, query: &str) -> Vec<String>{
        let mut cursor = self.query(&query).unwrap();
        let mut songs = Vec::new();
        while let Some(row) = cursor.next().unwrap() {
            let title = row[0].as_string().unwrap();
            songs.push(title.to_owned());
        }
        songs
    }

}
