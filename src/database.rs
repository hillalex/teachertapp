use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::models::{CreateSchool, School};
use crate::schema::schools::dsl::*;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn get_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migrations(db_url: &str) {
    let mut conn = get_connection(db_url);
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");
}

pub fn get_schools(database_url: &str) -> Result<Vec<School>, Error> {
    get_connection(database_url).transaction(|conn| {
        schools
            .load::<School>(conn)
    })
}

pub fn get_school(database_url: &str, school_id: i32) -> Result<School, Error> {
    get_connection(database_url).transaction(|conn| {
        schools.filter(id.eq(school_id))
            .select(School::as_select())
            .first(conn)
    })
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

pub fn delete_school(database_url: &str, school_id: i32) -> Result<usize, Error> {
    get_connection(database_url).transaction(|conn| {
        diesel::delete(schools)
            .filter(id.eq(school_id))
            .execute(conn)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestDatabase;

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

    #[test]
    fn can_get_schools() {
        let db = TestDatabase::new();
        let new_school = CreateSchool {
            name: "Saint Schoolson".to_string()
        };
        create_school(&db.url, &new_school)
            .expect("School created");

        let new_school = CreateSchool {
            name: "SchoolHill School".to_string()
        };
        create_school(&db.url, &new_school)
            .expect("School created");

        let res = get_schools(&db.url,)
            .expect("Retrieved schools");

        assert_eq!(res.len(), 2);
        assert_eq!(res.first().unwrap().name, "Saint Schoolson");
    }

    #[test]
    fn can_get_school_by_id() {
        let db = TestDatabase::new();
        let new_school = CreateSchool {
            name: "Saint Schoolson".to_string()
        };
        create_school(&db.url, &new_school)
            .expect("School created");

        let new_school = CreateSchool {
            name: "SchoolHill School".to_string()
        };
        create_school(&db.url, &new_school)
            .expect("School created");

        let res = get_school(&db.url, 2)
            .expect("Retrieved school");

        assert_eq!(res.id, 2);
        assert_eq!(res.name, "SchoolHill School");
    }

    #[test]
    fn can_delete_schools() {
        let db = TestDatabase::new();
        let new_school = CreateSchool {
            name: "Saint Schoolson".to_string()
        };
        create_school(&db.url, &new_school)
            .expect("School created");
        create_school(&db.url, &new_school)
            .expect("School created");

        let res = get_schools(&db.url,)
            .expect("Retrieved schools");

        assert_eq!(res.len(), 2);

        delete_school(&db.url, 1).unwrap();
        let res = get_schools(&db.url,)
            .expect("Retrieved schools");

        assert_eq!(res.len(), 1);
    }
}
