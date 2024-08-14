// This file is part of catabile.
//
// cantabile is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// cantabile is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with cantabile. If not,
// see <https://www.gnu.org/licenses/>.

use rusqlite::{Connection, Result};

mod constants {
    pub const CURRENT_DIR_ERROR_MESSAGE: &str = "Failed to establish current directory\
    and no path to the database was provided.";

    pub const CONNECTION_ERROR: &str = "Failed to get connection to database.\
    Does cantabile have readwrite permissions?";

    pub const CREATE_ALBUMS_TABLE: &str =
"CREATE TABLE IF NOT EXISTS Albums (
    path TEXT PRIMARY KEY,
    name TEXT,
    artists TEXT,
    year INTEGER,
    cover BLOB
) STRICT;
";

    pub const CREATE_TRACKS_TABLE: &str =
"CREATE TABLE IF NOT EXISTS Tracks (
    path TEXT PRIMARY KEY,
    album_path TEXT REFERENCES Albums (path),
    title TEXT,
    artists TEXT,
    genre TEXT,
    year INTEGER,
    position INTEGER
) STRICT;
";
}



/// Establish connection to the database file.
///
/// A path to the database can optionally be given. If it's not provided, then the path is
/// `CURRENT_DIRECTORY/database.sqlite`, where `CURRENT_DIRECTORY` is as given by
/// [`std::env::current_dir`].
///
/// # Panics
///
/// If no path is provided, then this function can panic if it fails to get the current directory.
/// See the error cases of [`std::env::current_dir`] for more details.
pub async fn init_db(path: Option<&str>) -> Result<Connection, rusqlite::Error> {
    let conn = if let Some(p) = path {
        // Make sure we have read/write permissions.
        {
            if let Ok(file) = std::fs::File::open(p) {
                assert!(
                    !file.metadata().unwrap().permissions().readonly(),
                    "{}",
                    constants::CONNECTION_ERROR,
                );
            }
        }

        Connection::open(p)?
    } else {
        let mut current_dir = std::env::current_dir()
            .expect(constants::CURRENT_DIR_ERROR_MESSAGE);
        current_dir.push("database.sqlite");
        Connection::open(current_dir)?
    };

    let _ = conn.execute(constants::CREATE_ALBUMS_TABLE, [])?;
    let _ = conn.execute(constants::CREATE_TRACKS_TABLE, [])?;

    Ok(conn)
}

pub async fn reset_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
    let _ = conn.execute("DELETE FROM Albums;", [])?;
    let _ = conn.execute("DELETE FROM Tracks;", [])?;
    Ok(())
}

#[cfg(test)]
pub async fn insert_dummy_value(conn: &Connection) -> Result<(), rusqlite::Error> {
    let _ = conn.execute(r#"INSERT INTO Albums VALUES (
        "some_path",
        "some_name",
        "some_artist",
        8564,
        "some_blob"
    );"#, [])?;
    Ok(())
}
