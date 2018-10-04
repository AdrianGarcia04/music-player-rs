use std::{fs::File, io::{Read, Error}};

pub fn create_database() -> Result<String, Error> {
    let mut tables_file = File::open("./tables.sql")?;
    let mut buffer = String::new();
    tables_file.read_to_string(&mut buffer)?;
    Ok(buffer)
}
