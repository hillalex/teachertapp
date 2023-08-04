use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;

use crate::models::{CreateSchool, School};
use crate::schema::schools::dsl::*;

pub fn get_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_school(database_url: &str, new_school: &CreateSchool) -> Result<School, Error> {
    get_connection(database_url).transaction(|conn| {
        diesel::insert_into(schools)
            .values(new_school)
            .execute(conn)?;

        schools
            .select(School::as_select())
            .order(id.desc())
            .first(conn)
    })
}

#[cfg(test)]
mod tests {
    use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
    use std::fs;
    use super::*;

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    fn run_db_migrations(conn: &mut impl MigrationHarness<diesel::sqlite::Sqlite>) {
        conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
    }

    struct TestDatabase {
        url: String,
    }

    impl TestDatabase {
        fn new() -> Self {
            let db_url = "test.sqlite";
            run_db_migrations(&mut get_connection(db_url));
            Self { url: db_url.to_string() }
        }
    }

    impl Drop for TestDatabase {
        fn drop(&mut self) {
            fs::remove_file(&self.url)
                .expect("Delete temp sqlite db");
        }
    }

    #[test]
    fn can_insert_school() {
        let db = TestDatabase::new();
        let new_school = CreateSchool {
            name: "Saint Schoolson".to_string()
        };
        let res = create_school(&db.url, &new_school)
            .expect("School created");
        assert_eq!(res.name, new_school.name);
        assert_eq!(res.id, 1);
    }
}
