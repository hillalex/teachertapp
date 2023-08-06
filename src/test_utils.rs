use tempfile::NamedTempFile;
use crate::database::run_migrations;

pub struct TestDatabase {
    pub url: String,
    // Keep this around so the NamedFile doesn't go out of scope
    // before the TestDatabase does. When they both go out of scope
    // the file will get cleaned up.
    #[allow(dead_code)]
    file: NamedTempFile,
}

impl TestDatabase {
    pub fn new() -> Self {
        let file = NamedTempFile::new().unwrap();
        let db_url = file.path().to_str().unwrap();
        run_migrations(db_url);
        Self {
            url: db_url.to_string(),
            file,
        }
    }
}
