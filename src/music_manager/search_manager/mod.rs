use super::{music_database::MusicDatabase};

pub struct SearchManager {
    results: Vec<String>,
    database: MusicDatabase,
}

impl SearchManager {

    pub fn new() -> SearchManager {
        let mut database = MusicDatabase::new();
        database.connect().unwrap();

        SearchManager {
            results: Vec::new(),
            database: database,
        }
    }

    pub fn set_rules(&mut self, query: &str) {
        let mut value = String::new();
        let mut search = String::from("SELECT rolas.title FROM rolas");
        let mut valid_search = false;
        for word in query.split_whitespace() {
            match word {
                "T:" => {
                    search += " WHERE rolas.title ";
                    valid_search = true;
                },
                "P:" => {
                    search += ", performers WHERE rolas.id_performer = performers.id_performer \
                    AND performers.name ";
                    valid_search = true;
                },
                "A:" => {
                    search += ", albums WHERE rolas.id_album = albums.id_album \
                    AND albums.name ";
                    valid_search = true;
                },
                "G:" => {
                    search += " WHERE rolas.genre ";
                    valid_search = true;
                },
                _ => {
                    value = format!("{} {}", value, word);
                }
            }
        };
        if valid_search {
            search += &format!("LIKE '%{}%'", value.trim());
            self.results = self.database.search_songs(&search);
        }
    }

    pub fn is_visible(&self, title: &str) -> bool {
        self.results.contains(&title.to_string())
    }

}
