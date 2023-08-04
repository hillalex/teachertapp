#[cfg(test)]
pub mod test_utils {
    use tempfile::NamedTempFile;

    use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
    use crate::schools::get_connection;

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    pub fn run_db_migrations(conn: &mut impl MigrationHarness<diesel::sqlite::Sqlite>) {
        conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
    }

    pub struct TestDatabase {
        pub url: String,
        // Keep this around so the NamedFile doesn't go out of scope
        // before the TestDatabase does. When they both go out of scope
        // the file will get cleaned up.
        file: NamedTempFile
    }

    impl TestDatabase {
        pub fn new() -> Self {
            let file = NamedTempFile::new().unwrap();
            let db_url = file.path().to_str().unwrap();
            run_db_migrations(&mut get_connection(db_url));
            Self {
                url: db_url.to_string(),
                file
            }
        }
    }
}