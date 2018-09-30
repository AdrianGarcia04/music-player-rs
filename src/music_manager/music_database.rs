use postgres::{Connection, TlsMode, error, types::ToSql, rows::Rows};
use std::io::{Error, ErrorKind};
type PostgresError = error::Error;

pub struct MusicDatabase {
    connection: Option<Connection>,
    username: Option<String>,
    password: Option<String>,
    host: Option<String>,
}

impl MusicDatabase {

    pub fn new() -> MusicDatabase {
        MusicDatabase {
            connection: None,
            username: None,
            password: None,
            host: None,
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
        let connection_URL = format!("postgresql://{}:{}@{}", username, password, host);
        self.connection = Some(Connection::connect(connection_URL, TlsMode::None)?);
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
}
