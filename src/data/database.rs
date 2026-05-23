use rusqlite::{Connection, Error};

pub trait Database {
    fn init(&self) -> Result<(), Error>;
    fn connect(database_path: &str) -> Result<Connection, Error>
    where
        Self: Sized;
}

pub struct FeedDatabase {
    conn: Connection,
}

pub struct StationDatabase {
    conn: Connection,
}

impl FeedDatabase {
    pub fn new(database_path: &str) -> FeedDatabase {
        FeedDatabase {
            conn: FeedDatabase::connect(database_path)
                .expect("ERROR: Could not open the feed database."),
        }
    }
}

impl StationDatabase {
    pub fn new(database_path: &str) -> StationDatabase {
        StationDatabase {
            conn: FeedDatabase::connect(database_path)
                .expect("ERROR: Could not open the station database."),
        }
    }
}

impl Database for FeedDatabase {
    fn init(&self) -> Result<(), Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS feeds (
                id TEXT UNIQUE PRIMARY KEY NOT NULL,
                title TEXT NOT NULL,
                url TEXT NOT NULL,
                description TEXT,
                content TEXT,
                author TEXT,
                category TEXT,
                pub_date DATETIME NOT NULL,
                source_feed_url TEXT,
                source_feed_title TEXT,
                is_read BOOLEAN DEFAULT 0,
                media_items TEXT,
                enclosures TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_feed_date ON feeds(pub_date)",
            [],
        )?;
        Ok(())
    }

    fn connect(database_path: &str) -> Result<Connection, Error> {
        let conn = Connection::open(database_path)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(conn)
    }
}

impl Database for StationDatabase {
    fn init(&self) -> Result<(), Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS stations (
                id TEXT UNIQUE PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                url TEXT NOT NULL,
                tags TEXT,
                country TEXT,
                country_code TEXT,
                language TEXT,
                language_code TEXT,
                votes NUMERIC,
                codec TEXT,
                bitrate NUMERIC,
                last_check_ok, BOOLEAN NOT NULL,
                last_change_time DATETIME NOT NULL,
                last_check_time DATETIME NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_station_check ON stations(last_check_time)",
            [],
        )?;
        Ok(())
    }

    fn connect(database_path: &str) -> Result<Connection, Error> {
        let conn = Connection::open(database_path)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn feed_database_init() {
        let temp_file =
            NamedTempFile::new().expect("Failed to create temp file for feed database.");
        let db = FeedDatabase::new(temp_file.path().to_str().unwrap());
        db.init().unwrap();
        assert!(db.conn.table_exists(None, "feeds").unwrap());
        assert!(db.conn.column_exists(None, "feeds", "id").unwrap());
        assert!(db.conn.column_exists(None, "feeds", "title").unwrap());
        assert!(db.conn.column_exists(None, "feeds", "url").unwrap());
        assert!(db.conn.column_exists(None, "feeds", "pub_date").unwrap());
    }

    #[test]
    fn station_database_init() {
        let temp_file =
            NamedTempFile::new().expect("Failed to create temp file for station database.");
        let db = StationDatabase::new(temp_file.path().to_str().unwrap());
        db.init().unwrap();
        assert!(db.conn.table_exists(None, "stations").unwrap());
        assert!(db.conn.column_exists(None, "stations", "id").unwrap());
        assert!(db.conn.column_exists(None, "stations", "name").unwrap());
        assert!(db.conn.column_exists(None, "stations", "url").unwrap());
    }
}
